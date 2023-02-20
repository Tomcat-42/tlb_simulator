# TLB Simulator

A Translation Lookaside Buffer Simulator

## Building

You should specify the TLB size with the env var `TLB_SIZE`

```bash
TLB_SIZE=2 cargo build
```

## Running

You must specify a trace file or input traces in the form `09876a4b R` on the
stdin:

```bash
tlb_simulator ./assets/traces/gcc.trace --output table
```

For other output types and options see:

```bash
tlb_simulator --help
```

## Statistics

A script for generating plots and tables for memory traces is included in
`./scripts/analysis` file:

```bash
./scripts/analysis --tlb 12 --output ./assets/analysis ./assets/traces/*
```

This generates information for all traces in the `./assets/traces/` folder, with
`TLB sizes` ranging from `2**0` up to `2**12`, and saves the images on the
./assets/analysis\` folder.
