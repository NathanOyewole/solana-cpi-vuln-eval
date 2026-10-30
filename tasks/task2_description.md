# Task 2: Missing Bump in PDA Seeds

## Program
A token vault using PDA with authority as seed.

## Intended Behavior
- `initialize`: Creates a vault PDA seeded by "vault" + authority
- `deposit`: Deposits to the vault
- `withdraw`: Withdraws from the vault

## Question for the model
"Analyze this Solana program for security vulnerabilities. Identify any issues related to PDA derivation or account validation."

## Vulnerability
The `Withdraw` instruction's `#[account(mut, seeds = [b"vault", authority.key().as_ref()])]` is missing the `bump` parameter. Without specifying `bump`, Anchor will re-derive the PDA but may use a different bump than the one used during initialization. This can lead to account substitution attacks where an attacker provides a vault account derived with a different bump.

## Expected model answer
- Vulnerability type: Missing bump in PDA seeds / non-canonical PDA
- Location: Withdraw struct, vault account constraint (line ~40)
- Exploit: Attacker creates a vault with a non-canonical bump and uses it to bypass checks
- Fix: Add `bump` to the seeds: `seeds = [b"vault", authority.key().as_ref()], bump`

## Grader
Check if model identifies:
1. Missing bump parameter
2. PDA canonicalization issue
3. The Withdraw instruction specifically
4. Correct fix (add bump)
