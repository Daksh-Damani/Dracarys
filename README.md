# Dracarys

Dracarys is a local AI Thingy that I'm building from scratch around a local LLM runtime.

The idea is pretty simple: instead of relying on a hosted AI service for everything, Dracarys should eventually have its own memory, knowledge, tools, and local intelligence while keeping the whole system under my control.

Right now, Dracarys uses **llama.cpp** as the inference backend and **Qwen3-4B** as the model.

This project is still very much a work in progress.

## What it can do right now

* Run completely locally
* Talk to a local GGUF model
* Use GPU acceleration through CUDA
* Maintain conversation history
* Save conversation history between sessions
* Load a custom system prompt
* Basic memory commands
* Rust-based application/backend



# Build llama.cpp with CUDA

Open a PowerShell terminal and go to the llama.cpp directory:

```powershell
cd E:\dracarys\runtime\llama.cpp
```

Create a build directory:

```powershell
cmake -B build -DGGML_CUDA=ON
```

Then build the Release version:

```powershell 
cmake --build build --config Release
```


# Get a GGUF model

Dracarys needs a GGUF model to actually generate responses.

For development, I'm currently using:

```text
Qwen3-4B-Q4_K_M.gguf
```

Place the model in /models


You can find compatible GGUF models on Hugging Face:

https://huggingface.co/models?search=GGUF

Make sure the model is actually a **GGUF language model** compatible with llama.cpp.



---

# Start llama-server

Go to the directory containing `llama-server.exe`.

For example:

```powershell
cd runtime\llama-build\bin\Release
```

Then start the server with your model:

```powershell
.\llama-server.exe `
    -m ".\models\Qwen3-4B-Q4_K_M.gguf" `
    -ngl 99 `
    --host 127.0.0.1 `
    --port 8080
```



#  Build Dracarys

Open another PowerShell window.

Build it:

```powershell
cargo build --release
```

If everything works, the executable will be:

```text
target/release/dracarys.exe
```

---

# Configure Dracarys

Before running it, make sure this file exists:

```text
config/system.txt
```

This contains Dracarys' system instructions.


Don't put secrets, API keys, passwords, or other private credentials in this file.

---

# Run Dracarys

Run the executable file and
You should see something similar to:

```text
DRACARYS
Local AI runtime
Backend: llama.cpp
Model: Qwen3-4B

Backend: online
Type /exit to quit.

You >
```

Now type something:

```text
You > Hello
```

Dracarys should respond using the local model.

---

# Basic commands

Currently available commands include:

```text
/exit
```

Closes Dracarys.

```text
/clear
```

Clears the current conversation.

```text
/memory
```

Shows saved conversation memory.

```text
/forget
```

Clears persistent memory.

```text
/remember <text>
```

Explicitly saves something to memory.

Example:

```text
You > /remember My favorite programming language is Rust.
```

---

# Development

Dracarys is currently being developed as a local-first system.

The current stack is roughly:

```text
Rust
  │
  ├── Dracarys Core
  ├── Memory
  ├── Knowledge
  └── Tools
       │
       ▼
   llama.cpp
       │
       ▼
   Local GGUF Model
```

The long-term goal is to make the model only one part of the system.

The planned architecture includes:

* Persistent long-term memory
* Knowledge retrieval
* Local document understanding
* Tool execution
* Better context management
* Multiple model support
* Internet access when explicitly enabled
* Multilingual interaction
* Eventually experimenting with our own smaller specialized model

For now, the priority is getting the core system stable before adding too much complexity.

---

# Current status

**Working:**

* [x] Rust application
* [x] llama.cpp backend
* [x] CUDA inference
* [x] GGUF model loading
* [x] Chat interface
* [x] System prompt
* [x] Conversation history
* [x] Persistent conversation storage
* [x] Basic memory commands

**Still being built:**

* [ ] Proper long-term memory
* [ ] Memory manager
* [ ] Knowledge/RAG system
* [ ] Tool system
* [ ] Better context management
* [ ] Web access
*    [ ] Multilingual improvements
* [ ] Custom model experimentation
* [ ] GUI

---

## A note about the model

Dracarys isn't its model.

Right now Qwen3-4B is being used as the underlying language model because it gives us a good local starting point.

The goal is to build the **Dracarys system around the model** so that the inference engine or model can eventually be replaced without rebuilding everything else.

That's the interesting part of this project.
