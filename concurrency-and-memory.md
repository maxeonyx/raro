# Concurrency and Memory

## Concurrency and Cancellation
- The most important element of concurrency is the ability to cancel work when needed.
- Python's `asyncio.CancelledError` is an excellent example: concurrency scopes should be able to cancel tasks yet still await graceful cleanup.
- We must offer comparable guarantees for synchronous code. That entails compiling in cancellation points, or selectively modifying instructions based on performance requirements. All code should be cancellable while still supporting graceful cleanup paths.

## Memory Use
- **Compile-time static sizes (preferred):** When a program is fully static with no heap allocation, we can allocate all required memory upfront by computing the types of stack objects.
- **Runtime static sizes (secondary):** Some sizes only become known after an initial phase (for example, configuration loading). As soon as the sizes are known, compute them and commit to allocations.
- **Per-object sizes (fallback):** At minimum we may know the size of individual objects.
- Introduce a hierarchy of traits for communicating these capabilities. Any object implementing compile-time static sizing must also satisfy the broader traits.

## Memory Allocation Strategy
- The interpreter/runtime should enforce a soft memory limit. When a memory error occurs, it raises the limit and surfaces an application-level memory error so cleanup can proceed with the additional memory.
- Because we prefer to allocate memory early using the sizing traits, we expose memory pressure as soon as possible.
- Design APIs that encourage programmers to constrain dynamic components (e.g., number of request handlers). If each handler's memory footprint is known and bounded, we can determine the total memory requirements for components such as a web server.

## Stack Management
- Treat coroutines as ordinary stacks that can be swapped, including across threads. (Open question: in Rust, work stealing inhibits patterns that cross await points. We may require opt-in for work-stealing pools. By default, tasks should remain on their originating thread even if the thread hosts multiple tasks.)
- Provide Python-like `contextlib`-style contexts that travel with stacks as they move between thread-locals.
- Manage synchronous stacks—and ideally the runtime's main stack—using the same mechanisms. Avoid relying on a conventional C stack for the runtime.
