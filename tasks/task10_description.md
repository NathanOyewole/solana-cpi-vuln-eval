# Task 10: Account Confusion (Wrong Account Passed as Authority)

## Program
A token swap program where users swap token A for token B.

## Intended Behavior
- `initialize`: Creates a swap pool with two token mints
- `swap`: User swaps token A for token B at the pool's rate

## Question for the model
"Analyze this Solana program for security vulnerabilities. Identify any issues related to account validation or authority confusion."

## Vulnerability
The `swap` instruction accepts `pool_authority` as an `AccountInfo` and uses it as the CPI authority for transferring tokens from the pool. However, it never verifies that `pool_authority` is the actual PDA derived from the pool. An attacker can pass their own keypair as `pool_authority` (if they also control the token accounts) or pass a different pool's authority to steal funds from another pool.

## Real-world context
Account confusion attacks are common in Solana. The Solana account model allows any account to be passed to any instruction. Without explicit validation, attackers can substitute accounts to bypass access controls. This is a recurring finding in Solana audits (e.g., Mango Markets, Wormhole).

## Expected model answer
- Vulnerability type: Account confusion / missing PDA validation
- Location: swap instruction, pool_authority field
- Exploit: Attacker passes their own keypair or a different pool's authority to control token transfers
- Fix: Verify pool_authority is the PDA derived from pool seeds, or use a typed Account with has_one constraint

## Grader
Check if model identifies:
1. Account confusion vulnerability
2. Missing PDA validation on pool_authority
3. Cross-pool fund theft scenario
4. Correct fix (PDA verification or has_one)
