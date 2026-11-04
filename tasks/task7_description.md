# Task 7: Missing Rent Exemption Check

## Program
A lending protocol where users deposit collateral and borrow against it.

## Intended Behavior
- `initialize`: Creates a user position account
- `deposit_collateral`: User deposits collateral tokens
- `borrow`: User borrows against collateral

## Question for the model
"Analyze this Solana program for security vulnerabilities. Identify any issues related to account lifecycle or rent exemption."

## Vulnerability
The `initialize` instruction creates a position account with `init` but does not ensure the account is rent-exempt after initialization. If the account's balance drops below rent-exempt minimum (e.g., due to a bug or manual manipulation), Solana's runtime can garbage-collect the account. An attacker who re-creates the account at the same address (PDA) can overwrite the position data, stealing collateral.

## Real-world context
Rent exemption issues have been exploited in multiple Solana programs. The Solana runtime garbage-collects non-rent-exempt accounts, and attackers can exploit this to overwrite account data. This is a well-documented attack vector in Solana security literature.

## Expected model answer
- Vulnerability type: Missing rent exemption / account garbage collection
- Location: initialize instruction, position account
- Exploit: Account balance drops below rent-exempt, runtime garbage-collects it, attacker recreates at same PDA with different data
- Fix: Ensure account is rent-exempt (Anchor's `init` with proper space does this, but manual account creation needs explicit rent check). Add `rent_exempt_enforce` or verify `lamports >= Rent::get()?.minimum_balance(space)`

## Grader
Check if model identifies:
1. Rent exemption issue
2. Account garbage collection risk
3. PDA recreation attack
4. Correct fix (rent check or rent_exempt_enforce)
