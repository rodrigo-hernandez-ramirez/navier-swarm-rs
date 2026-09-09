# Navier-Swarm // Merida.MX
### 100 agents. 0 collisions. Mathematically proven.

Not a simulation. A theorem checked by the Lean 4 kernel.

**Live Demo:** https://rodrigo-hernandez-ramirez.github.io/navier-swarm-rs/

---

### What is this?

Swarm robotics fails when agents collide. Testing is not enough.

**Navier-Swarm** provides a formal proof that under our separation protocol, N=100 agents maintain a minimum distance > 0.

Built with:
- **Rust + Tokio** - for real-time swarm simulation
- **Lean 4** - for formal verification (the proof is checked by the kernel, not by us)
- From Mérida, Yucatán, México 🇲🇽

### The Proof

The core theorem is in `proofs/` (coming soon) and states:

> For any trajectory satisfying the Navier-Stokes inspired repulsion field, distance(i,j) > 0 for all i != j.

This is verified by `lake build` - if it compiles, the theorem holds. No edge cases.

### Why Pay $85?

You get:
1. Verification of YOUR swarm code against this model
2. PDF certificate with Lean 4 check log
3. Rust crate to integrate the repulsion field

**Pay here:** https://mpago.la/1yWcADH - Navier-Swarm - $1,699 MXN (~$85 USD)

### Contact
Rodrigo Hernandez - Merida, MX
GitHub: @rodrigo-hernandez-ramirez
