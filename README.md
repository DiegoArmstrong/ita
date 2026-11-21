# ita
A programming language that prioritizes the little things: bits, registers, microseconds, and more.


# The ita Manifesto!

## Zero Hidden Costs, Explicit Allocaiton

 There is no garbage collector, no hidden heap allocation, and no invisible runtime overhead. Every byte of RAM and every clock cycle must be accountable at compile time.

## Bits and Wires Are First-Class Citizens

Manipulating bit-fields, hardware registers, and wire endianness must be native syntax, never manual bit-shifting or pointer casting.

## Physical Reality Belongs in the Type System

Numbers representing time, voltage, frequency, or hardware pins carry their physical meaning in their types.

## Timeouts and Failures Are Mandatory Paths

Every external I/O operation can fail or hang, so the language refuses to compile I/O calls that ignore errors or lack a deadline.

## Zero-Friction C Interoperability 

Calling C code, linking vendor SDKs, and importing C header files must require zero wrapper boilerplate or FFI translation layers.

## Boring, Readable, No-Magic Syntax

ita must be obvious to read. When reading ita, you should be able to understand exactly what it physically represents.

## Protocol First, Hardware Agnostic Standard Library

The standard library ships with universal, zero-allocation interfaces for buses (`SPI`, `I2C`, `UART`, `CAN`, `TCP/UDP`), framing (`COBS`, `SLIP`, `CRC`), and deterministic buffers (`RingBuffer`); completely decoupled from specific chip vendors or OS platforms.