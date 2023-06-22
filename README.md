# Browser Top

**Browser Top** is a simple project that shows CPU usage of the serve in a browser with the help of websockets.

In this project, I learnt about how websockets work, how they are implemented in rust (specifically actix-web framework) and how to run synchronous in async environment with the help of tokio's signals and channels.

## Required dependencies

This project requires a few dependencies to be installed in order to work:

- **Make** - Comes installed with most Unix systems and also comes with Microsoft Visual Studio for C++
- **Node.js and yarn** - Install from https://nodejs.org/en or your distribution's package manager
- **Rust** - Install easily from https://rustup.rs/ for the latest version or your distribution's package manager.

## Instructions

Install all the required node.js dependencies using:
```bash
yarn --cwd frontend install
```

### Development version

To run the development version of this application with hot reloading, run:

```bash
make dev
```

### Release version

To run the release(optimized) version of this application, run:

```bash
make start
```
