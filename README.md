# Shared Memory Game Server Status Monitoring Demo

This demo illustrates how to monitor the real-time status of a game server using **POSIX Shared Memory**. In this demonstration, We monitor a **pseudo** `user_count` that increments by one with each server tick.

## Purpose and Utility of this Demo

This approach is particularly useful in environments where **IO resource efficiency is critical, such as game servers**. By sharing status information through direct **memory access** rather than traditional IO operations, more resources can be dedicated to the core functionalities of the game server.

For instance, this can be effectively used to monitor a variety of server metrics like current connected users, RPC calls per minute, or overall server load.

## Lock-Free Status Monitoring (Dirty Read)

For status values that can be represented within a **single word size (e.g., `u32`, `u64`, `usize`)**, you can achieve safe read/write operations without the need for traditional locks like **Mutexes**, by utilizing **Atomic Operations or `volatile` memory accesses**.

While this involves what is sometimes referred to as a "Dirty Read", it ensures that single-word writes and reads are atomic at the hardware level. This minimizes data inconsistencies and allows for **maximum performance**.

## How to Run the Demo

Follow these steps to build and run the shared memory demo:

1. **Open Dev Container**

   Open the project in a VS Code Dev Container.

2. **Build Binaries**

   Compile both the server and monitor binaries.

   ```sh
   cargo build
   ```

3. **Run Server Process**

   Start the server process in your terminal. This process will create and continuously write to the shared memory.

   ```sh
   ./target/debug/server
   ```

   You should see the server output:

   ```sh
   Hello, I'm a server!
   ```

4. **Run Monitor Process**

   Open a **separate terminal** and run the monitor process. This process will read data from the shared memory.

   ```sh
   ./target/debug/monitor
   ```

   You'll see the monitor displaying the `user_count` as it reads from shared memory:

   ```sh
   Hello, I'm a monitor!
   Server status: ServerStatus { user_count: 7 }
   Server status: ServerStatus { user_count: 9 }
   Server status: ServerStatus { user_count: 11 }
   ...
   ```

5. **Stop Processes**

   To stop either process, press `Ctrl+C` in its respective terminal. You'll see a graceful shutdown message, indicating that shared memory resources are being unmapped and unlinked.

   ```sh
   ^CReceived Ctrl+C signal.
   ```
