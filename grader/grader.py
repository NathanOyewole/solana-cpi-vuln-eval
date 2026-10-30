#!/usr/bin/env python3
"""Grader for Solana CPI/signer-check vulnerability detection eval."""

import json
import os
import sys

TASKS_DIR = os.path.join(os.path.dirname(__file__), "..", "tasks")

# Keywords that indicate correct vulnerability identification per task
EXPECTED = {
    "task1": {
        "vuln_keywords": ["signer", "has_one", "AccountInfo", "not signed"],
        "location_keywords": ["authority", "Withdraw"],
        "fix_keywords": ["Signer", "Signer<'info>"],
    },
    "task2": {
        "vuln_keywords": ["bump", "PDA", "canonical", "seeds"],
        "location_keywords": ["Withdraw", "vault"],
        "fix_keywords": ["bump", "seeds = [b\"vault\""],
    },
    "task3": {
        "vuln_keywords": ["unchecked", "AccountInfo", "ownership", "validation"],
        "location_keywords": ["Withdraw", "vault_token", "recipient_token"],
        "fix_keywords": ["Account<", "TokenAccount", "ownership"],
    },
    "task4": {
        "vuln_keywords": ["reentrancy", "state update", "external call", "CPI"],
        "location_keywords": ["withdraw", "balance"],
        "fix_keywords": ["checks-effects-interactions", "before", "update first"],
    },
    "task5": {
        "vuln_keywords": ["has_one", "access control", "authority", "constraint"],
        "location_keywords": ["Withdraw", "SetAuthority", "vault"],
        "fix_keywords": ["has_one = authority", "has_one"],
    },
}

def grade_response(task_id, model_response):
    """Grade a model's response for a given task."""
    exp = EXPECTED.get(task_id, {})
    resp_lower = model_response.lower()

    scores = {
        "vuln_identified": any(k.lower() in resp_lower for k in exp.get("vuln_keywords", [])),
        "location_correct": any(k.lower() in resp_lower for k in exp.get("location_keywords", [])),
        "fix_correct": any(k.lower() in resp_lower for k in exp.get("fix_keywords", [])),
    }

    total = sum(scores.values())
    max_score = len(scores)
    return {
        "task_id": task_id,
        "scores": scores,
        "score": total,
        "max": max_score,
        "passed": total == max_score,
    }

def main():
    if len(sys.argv) < 3:
        print("Usage: grader.py <task_id> <response_file>")
        sys.exit(1)

    task_id = sys.argv[1]
    with open(sys.argv[2]) as f:
        response = f.read()

    result = grade_response(task_id, response)
    print(json.dumps(result, indent=2))

if __name__ == "__main__":
    main()
