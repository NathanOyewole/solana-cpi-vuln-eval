#!/usr/bin/env python3
"""Grader for Solana vulnerability detection eval.

Supports 10 tasks covering common Solana security vulnerabilities.
Uses keyword-based scoring with weighted categories.
"""

import json
import os
import sys

TASKS_DIR = os.path.join(os.path.dirname(__file__), "..", "tasks")

EXPECTED = {
    "task1": {
        "name": "Missing signer check",
        "vuln_keywords": ["signer", "has_one", "not signed", "authority"],
        "location_keywords": ["authority", "withdraw", "admin"],
        "exploit_keywords": ["unauthorized", "anyone", "without signing", "drain"],
        "fix_keywords": ["Signer", "Signer<'info>", "has_one", "constraint"],
        "weight": {"vuln": 1, "location": 1, "exploit": 1, "fix": 1},
    },
    "task2": {
        "name": "PDA seed collision (missing bump)",
        "vuln_keywords": ["bump", "PDA", "canonical", "seeds", "collision"],
        "location_keywords": ["withdraw", "vault", "pda"],
        "exploit_keywords": ["collision", "fake", "recreate", "different bump"],
        "fix_keywords": ["bump", "seeds = [", "canonical bump", "find_program_address"],
        "weight": {"vuln": 1, "location": 1, "exploit": 1, "fix": 1},
    },
    "task3": {
        "name": "Unchecked account ownership",
        "vuln_keywords": ["unchecked", "AccountInfo", "ownership", "validation"],
        "location_keywords": ["withdraw", "vault_token", "recipient_token", "token"],
        "exploit_keywords": ["fake account", "attacker", "wrong owner", "inject"],
        "fix_keywords": ["Account<", "TokenAccount", "ownership", "constraint"],
        "weight": {"vuln": 1, "location": 1, "exploit": 1, "fix": 1},
    },
    "task4": {
        "name": "Reentrancy via CPI",
        "vuln_keywords": ["reentrancy", "state update", "external call", "CPI", "checks-effects"],
        "location_keywords": ["withdraw", "balance", "transfer"],
        "exploit_keywords": ["re-enter", "drain", "before update", "reentrancy"],
        "fix_keywords": ["checks-effects-interactions", "update first", "before", "reentrancy guard"],
        "weight": {"vuln": 1, "location": 1, "exploit": 1, "fix": 1},
    },
    "task5": {
        "name": "Missing has_one constraint",
        "vuln_keywords": ["has_one", "access control", "authority", "constraint"],
        "location_keywords": ["withdraw", "set_authority", "vault", "authority"],
        "exploit_keywords": ["unauthorized", "any authority", "bypass", "access control"],
        "fix_keywords": ["has_one = authority", "has_one", "constraint"],
        "weight": {"vuln": 1, "location": 1, "exploit": 1, "fix": 1},
    },
    "task6": {
        "name": "Token account mint mismatch",
        "vuln_keywords": ["mint", "mismatch", "token account", "validation"],
        "location_keywords": ["claim_rewards", "vault_reward", "user_reward", "token"],
        "exploit_keywords": ["wrong mint", "drain", "fake token", "mint mismatch"],
        "fix_keywords": ["mint =", "constraint", "TokenAccount", "mint constraint"],
        "weight": {"vuln": 1, "location": 1, "exploit": 1, "fix": 1},
    },
    "task7": {
        "name": "Missing rent exemption",
        "vuln_keywords": ["rent", "exemption", "garbage collection", "account"],
        "location_keywords": ["initialize", "position", "account"],
        "exploit_keywords": ["garbage collect", "recreate", "overwrite", "PDA"],
        "fix_keywords": ["rent_exempt", "minimum_balance", "Rent::get", "space"],
        "weight": {"vuln": 1, "location": 1, "exploit": 1, "fix": 1},
    },
    "task8": {
        "name": "Unsafe deserialization",
        "vuln_keywords": ["deserialization", "ownership", "AccountInfo", "unchecked"],
        "location_keywords": ["buy_nft", "listing", "account"],
        "exploit_keywords": ["fake account", "inject", "crafted data", "Wormhole"],
        "fix_keywords": ["Account<", "owner check", "try_deserialize", "ownership"],
        "weight": {"vuln": 1, "location": 1, "exploit": 1, "fix": 1},
    },
    "task9": {
        "name": "Signer privilege escalation",
        "vuln_keywords": ["privilege escalation", "invoke_signed", "approval", "duplicate"],
        "location_keywords": ["execute", "approvals", "multisig", "threshold"],
        "exploit_keywords": ["duplicate", "non-owner", "inflate", "bypass"],
        "fix_keywords": ["unique", "owner set", "threshold", "bitmap"],
        "weight": {"vuln": 1, "location": 1, "exploit": 1, "fix": 1},
    },
    "task10": {
        "name": "Account confusion",
        "vuln_keywords": ["account confusion", "PDA", "validation", "authority"],
        "location_keywords": ["swap", "pool_authority", "authority"],
        "exploit_keywords": ["substitute", "different pool", "steal", "cross-pool"],
        "fix_keywords": ["PDA", "has_one", "verify", "derive"],
        "weight": {"vuln": 1, "location": 1, "exploit": 1, "fix": 1},
    },
}


def grade_response(task_id, model_response):
    exp = EXPECTED.get(task_id, {})
    if not exp:
        return {"error": f"Unknown task: {task_id}"}

    resp_lower = model_response.lower()

    def check_keywords(keywords):
        return any(k.lower() in resp_lower for k in keywords)

    scores = {
        "vuln_identified": check_keywords(exp.get("vuln_keywords", [])),
        "location_correct": check_keywords(exp.get("location_keywords", [])),
        "exploit_described": check_keywords(exp.get("exploit_keywords", [])),
        "fix_correct": check_keywords(exp.get("fix_keywords", [])),
    }

    total = sum(scores.values())
    max_score = len(scores)
    return {
        "task_id": task_id,
        "task_name": exp["name"],
        "scores": scores,
        "score": total,
        "max": max_score,
        "percentage": round(total / max_score * 100, 1),
        "passed": total == max_score,
    }


def grade_all(responses_dir):
    """Grade all responses in a directory. Expects files named taskN.txt or taskN.md."""
    results = []
    for task_id in sorted(EXPECTED.keys()):
        for ext in [".txt", ".md", ".json"]:
            path = os.path.join(responses_dir, f"{task_id}{ext}")
            if os.path.exists(path):
                with open(path) as f:
                    response = f.read()
                result = grade_response(task_id, response)
                results.append(result)
                break
        else:
            results.append({"task_id": task_id, "error": "No response file found"})

    total_score = sum(r.get("score", 0) for r in results if "score" in r)
    total_max = sum(r.get("max", 0) for r in results if "max" in r)
    return {
        "results": results,
        "total_score": total_score,
        "total_max": total_max,
        "percentage": round(total_score / total_max * 100, 1) if total_max > 0 else 0,
    }


def main():
    if len(sys.argv) < 2:
        print("Usage:")
        print("  grader.py <task_id> <response_file>     # grade single task")
        print("  grader.py --all <responses_dir>         # grade all tasks")
        sys.exit(1)

    if sys.argv[1] == "--all":
        if len(sys.argv) < 3:
            print("Usage: grader.py --all <responses_dir>")
            sys.exit(1)
        results = grade_all(sys.argv[2])
        print(json.dumps(results, indent=2))
    else:
        task_id = sys.argv[1]
        with open(sys.argv[2]) as f:
            response = f.read()
        result = grade_response(task_id, response)
        print(json.dumps(result, indent=2))


if __name__ == "__main__":
    main()