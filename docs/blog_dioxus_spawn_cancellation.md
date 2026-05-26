# Dioxus 0.7 踩坑：为什么 spawn 的闭包不执行？

最近在用 Dioxus 0.7 写一个全栈看板应用，技术栈是 Dioxus 0.7 WASM + Axum + SQLite + JWT 做登录。前端是一个四列看板，任务卡片上有左右箭头按钮，点击后把任务移动到相邻列。为了体验，我用了乐观更新：先改本地的 Signal，状态立刻反映在界面上，再异步调后端持久化。

结果遇到了一个让人抓狂的 bug：点击左右移动按钮后，`spawn` 里面的后端请求闭包根本不执行，但点击 Claim 按钮的 `spawn` 完全正常。

三个按钮的外层 onclick 闭包都能正常触发——我在关键位置打的 `info!()` 都能看到。但左右移动按钮里面那个 `spawn(async move { ... })` 的闭包就是不执行，连里面第一行的日志都打不出来。Claim 按钮和左右移动按钮的代码结构几乎一模一样，就改了一两行数据准备逻辑，为什么一个能跑一个不能？

排查了两天。先怀疑是变量捕获问题，但三个按钮的 clone 模式完全一致，所有权没问题。又怀疑 `save_task` 的序列化有坑，可 spawn 内部第一行就该打印的 info 都没出现，说明整个 future 根本没被 poll。还试了加 `e.stop_propagation()` 排除事件冒泡，同样没用。

关键时刻在 spawn 前后各加了一行日志：

```rust
onclick: move |_| {
    info!("[outer] before spawn");          // ✅ 打了
    spawn(async move {
        info!("[inner] spawn started");     // ❌ 没打
        let _ = save_task(updated).await;
    });
    info!("[outer] after spawn");           // ✅ 打了
}
```

spawn 只是把 future 丢给调度器然后立刻返回，本身不阻塞，所以前后两行都能打印。但 future 里面的代码永远没被 poll。

这时我突然想到：Claim 按钮改的是 `assignee`，左右移动按钮改的是 `status`。改 `status` 会让任务卡片**移动到另一列**——这意味着当前 `TaskCard` 组件会被卸载。会不会跟这个有关？

查了 Dioxus 0.7 的 `spawn` 实现，果然。spawn 会调用 `current_scope_id()` 拿到当前组件的 scope，然后把 future 绑在这个 scope 的任务队列上。当 scope 被销毁时，所有未完成的 task 都会被取消。

而事件处理的执行时序是这样的：用户点击 ◀，onclick 闭包执行，里面先写 Signal 触发乐观更新（标记组件 dirty），然后 spawn 把 future 注册到当前 scope 的任务队列。闭包返回后，Dioxus 马上处理脏组件，开始重渲染看板。任务因为 status 变了，从 Todo 列移到 InProgress 列，旧列的 `TaskCard` 被卸载，scope 销毁，刚才注册的 future 还没来得及被 poll 就被干掉了。新列的 `TaskCard` 创建出来，风平浪静，好像什么都没发生——除了后端没收到请求。

Claim 按钮之所以正常，是因为它只改 `assignee` 不改 `status`，任务不会移动到别的列，`TaskCard` 保持挂载，scope 一直有效，future 能正常被 poll。

所以核心问题是 `tasks.write()` 触发的乐观更新导致组件立即卸载，scope 被销毁，还没来得及轮询的 future 随之取消。而 Dioxus 这样设计是有道理的：框架需要知道哪些异步任务属于哪个组件，以便在组件卸载时自动清理，避免内存泄漏。React 的 useEffect cleanup 和 Vue 的 onUnmounted 本质上干的是同一件事。大多数场景下（数据请求、定时器、事件监听）这种自动清理非常有用，但当异步操作需要超越组件生命周期时——比如把数据同步到后端——就需要换一种思路了。

第一个方案最直接：去掉乐观更新，把 `tasks.write()` 移到 spawn 里面，等后端响应后再改 UI。能工作，但失去即时反馈，用户点完有明显延迟感。

第二个方案把 spawn 逻辑提升到父组件。父组件看板页面永远不会被卸载，它的 scope 一直有效，spawn 不会被取消。通过 EventHandler 回调把操作传给父组件，架构上更干净，但需要改动多个文件。

第三个方案是 `wasm_bindgen_futures::spawn_local`。写一个平台适配的 helper，在 wasm 目标上直接用浏览器原生的微任务队列调度 future，完全绕开 Dioxus 的 scope 系统，组件卸载也不影响。服务端回退到 dioxus::spawn（onclick 本来就不在服务端执行）。这个方案改动最小，保留了乐观更新，前后端都兼容。但它需要额外依赖 `wasm-bindgen-futures` 和条件编译，有点侵入式。

真正干净的做法，是走通 `use_coroutine` 这条路。

Dioxus 0.7 提供了 `use_coroutine`，可以创建一个持久的后台协程，绑定在调用它的组件的 scope 上。在父组件（Todos，看板页面）里用它创建一个协程，负责处理所有后端保存请求：

```rust
// 在 Todos 组件中，Todos 永不卸载
let _save_tx = use_coroutine(|mut rx: UnboundedReceiver<Task>| async move {
    while let Some(task) = rx.next().await {
        let _ = save_task(task).await;
    }
});
```

协程绑在 Todos 的 scope 上，Todos 永远不被卸载，所以这个协程会一直活着。子组件怎么拿到发送端呢？答案是 `use_coroutine_handle`。它相当于 coroutine 版的 `use_context`——父组件通过 `use_coroutine` 注入了协程，后代组件不管嵌套多少层，只要按泛型类型就能拿到 handle，不需要逐层传 props：

```rust
// TaskCard 中，无需 props 传递
let save_tx = use_coroutine_handle::<Task>();
```

然后 onclick 闭包就回到了最简洁的写法：乐观更新同步写 Signal，后端持久化同步 `send` 到协程，两行都是同步操作，不涉及任何 spawn，也就不存在 scope 被取消的问题：

```rust
onclick: move |_| {
    let mut updated = tasks.read().iter().find(|t| t.id == task_id).cloned().unwrap();
    // ... 修改 updated ...
    tasks.write().iter_mut()...;  // 乐观更新
    save_tx.send(updated);         // 扔给协程处理
}
```

`Coroutine::send()` 是纯同步的方法调用。不经过 spawn，不依赖当前在哪个组件的 scope 里，即使这个 `TaskCard` 下一毫秒就被卸载了，协程也会妥善完成 `save_task` 请求。而且 `save_tx.send()` 背后是一个无界通道，协程按 FIFO 顺序依次处理，不会出现并发写入的竞态问题。

回看整个踩坑过程，解决方案的演进其实体现了一个很典型的思路：遇到框架的"贴心设计"挡路时，第一反应往往是绕过它（spawn_detached 绕开 scope），但更好的方式是利用框架提供的正确抽象（coroutine）在更高层级解决问题。`use_coroutine` 本来就是 Dioxus 为这类场景设计的——后台任务、消息处理、需要跨越组件生命周期的异步操作。把它和 `use_coroutine_handle` 配合使用，既不需要额外的 crate，也不需要条件编译，代码结构还更清晰：父组件管理异步生命周期，子组件只负责触发。

总结一下排查这类问题的通用思路。如果你在 Dioxus 0.7 中遇到 spawn 的闭包不执行，先在 spawn 内部第一行加日志，确认 future 有没有被 poll 过。然后检查 spawn 之前是不是有 `Signal.write()`，写入会不会触发重渲染导致当前组件被卸载——尤其注意列表元素位置变化、条件渲染分支变化、状态改变导致元素在不同容器间移动这些场景。如果都命中，就别硬塞 spawn 了，试试 `use_coroutine` + `use_coroutine_handle`，或者至少把 spawn 移到不会被卸载的父组件里。