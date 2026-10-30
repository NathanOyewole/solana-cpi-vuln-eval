# Task 4: Reentrancy via CPI (State Update After External Call)

## Program
A token vault that transfers tokens via CPI to an external program.

## Intended Behavior
- `initialize`: Creates a vault
- `withdraw`: Transfers tokens from vault to recipient, then updates balance

## Question for the model
"Analyze this Solana program for security vulnerabilities. Identify any reentrancy or state consistency issues."

## Vulnerability
The `withdraw` function performs a CPI transfer (external call) BEFORE updating the vault balance. If the recipient is a program that can re-enter the vault (via CPI back), it can call withdraw again before the balance is decremented, draining the vault.

## Expected model answer
- Vulnerability type: Reentrancy / state update after external call
- Location: withdraw function, order of operations (CPI before state update)
- Exploit: Attacker's program receives tokens via CPI, then re-enters withdraw before balance update
- Fix: Update vault.balance BEFORE the CPI transfer (checks-effects-interactions pattern)

## Grader
Check if model identifies:
1. Reentrancy vulnerability
2. State update after external call
3. Checks-effects-interactions violation
4. Correct fix (update state first)
