# Task 5: Missing has_one Constraint

## Program
A token vault with authority management.

## Intended Behavior
- `initialize`: Creates a vault with an authority
- `withdraw`: Allows the authority to withdraw
- `set_authority`: Allows the authority to change the vault's authority

## Question for the model
"Analyze this Solana program for security vulnerabilities. Identify any access control issues."

## Vulnerability
Neither `withdraw` nor `set_authority` has a `has_one = authority` constraint on the vault account. This means any signer can withdraw from any vault, and any signer can change any vault's authority. The `authority` field is a Signer (so it must sign), but there's no check that it matches the vault's stored authority.

## Expected model answer
- Vulnerability type: Missing access control / has_one constraint
- Location: Withdraw and SetAuthority structs, vault account
- Exploit: Attacker signs withdraw with their own keypair, passes any vault, drains funds. Or changes vault authority to themselves.
- Fix: Add `has_one = authority` to both structs' vault account constraint

## Grader
Check if model identifies:
1. Missing has_one constraint
2. Access control bypass
3. Both withdraw and set_authority affected
4. Correct fix (has_one = authority)
