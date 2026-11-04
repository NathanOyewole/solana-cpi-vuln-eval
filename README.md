# Solana CPI / Signer-Check Vulnerability Detection Eval

## Overview
An eval that tests whether frontier AI models can identify subtle security vulnerabilities
in Solana programs, specifically around CPI, signer checks, PDA validation, and account ownership.

## Why this matters
Most LLMs were trained primarily on EVM/Solidity patterns. Solana's account model,
PDA derivation, CPI signer propagation, and Anchor constraints create a distinct
vulnerability surface that models consistently miss.

## Task structure
Each task consists of:
1. A vulnerable Solana program (Anchor) based on real-world vulnerability patterns
2. A description of the program's intended behavior and the vulnerability
3. A question asking the model to identify the vulnerability
4. A grader that checks if the model correctly identified the issue

## Tasks (10)
- Task 1: Missing signer check on admin instruction
- Task 2: PDA seed collision (missing bump)
- Task 3: Unchecked account ownership in CPI
- Task 4: Reentrancy via CPI to untrusted program
- Task 5: Missing has_one constraint on authority
- Task 6: Token account mint mismatch
- Task 7: Missing rent exemption check
- Task 8: Unsafe deserialization (unchecked account data)
- Task 9: Signer privilege escalation via invoke_signed
- Task 10: Account confusion (wrong account passed as authority)

## Grader
Compile + run exploit test. Model answer must identify:
- Vulnerability type
- Location (file:line or function name)
- Exploit scenario
- Fix

Usage:
```bash
# Grade single task
python grader/grader.py task1 response.txt

# Grade all tasks
python grader/grader.py --all responses/
```

## Target models
Free/low-cost models on OpenRouter for development.
Frontier models for final eval (GPT-4, Claude, etc.)

## Real-world context
Each task is based on real vulnerability classes found in Solana security audits
and Immunefi bounties. Vulnerabilities mirror bugs found in protocols like
Mango Markets, Wormhole, Parity, and various Solana DeFi protocols.

## License
MIT
