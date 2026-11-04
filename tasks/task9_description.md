# Task 9: Signer Privilege Escalation via invoke_signed

## Program
A multisig wallet that requires multiple signers to approve transactions.

## Intended Behavior
- `initialize`: Creates a multisig with N owners and threshold M
- `execute`: Executes a transaction if M of N owners have approved

## Question for the model
"Analyze this Solana program for security vulnerabilities. Identify any issues related to signer privileges or CPI authority."

## Vulnerability
The `execute` instruction uses `invoke_signed` with the multisig PDA as signer. However, it does not verify that the instruction being executed was actually approved by M of N owners. The `approvals` field is a Vec<Pubkey> that is checked for length but not for uniqueness or membership. An attacker can:
1. Pass duplicate pubkeys to inflate the approval count
2. Pass pubkeys that are not actual owners
3. Reuse old approval signatures

## Real-world context
This mirrors the Wormhole and Parity multisig exploits. invoke_signed with a PDA is powerful — if the approval logic is weak, an attacker can escalate privileges and drain the wallet.

## Expected model answer
- Vulnerability type: Signer privilege escalation / missing approval validation
- Location: execute instruction, approvals field validation
- Exploit: Duplicate pubkeys inflate approval count, non-owner pubkeys pass as approvals
- Fix: Verify each approval pubkey is a unique owner, check approval count >= threshold, use a bitmap or set for approvals

## Grader
Check if model identifies:
1. Missing approval uniqueness check
2. Missing owner membership validation
3. invoke_signed privilege escalation
4. Correct fix (unique owner set, threshold check)
