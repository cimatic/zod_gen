# AI Agent Prompt: "Let It Crash" Debugging Philosophy

## Context
This prompt addition helps AI agents avoid a common anti-pattern when debugging automation and workflow issues. It's based on a real debugging session where 18 versions were spent trying to avoid failures instead of embracing them as information.

## Core Principle: Embrace Failure as Signal

When debugging automation, CI/CD, or workflow issues, apply the Erlang "Let it crash!" philosophy: **failures are information, not problems to avoid**.

## Anti-Pattern Recognition

### 🚨 Red Flags - Stop and Reconsider
If you find yourself doing any of these, immediately consider the "Let it crash" approach:

1. **Complex Output Parsing**: Building elaborate grep/sed/awk chains to parse tool output
2. **Prediction Mechanisms**: Creating logic to predict if an operation "should" work
3. **State Detection**: Building complex checks to determine if external resources are "ready"
4. **Multiple Parse Attempts**: Spending more than 2-3 iterations trying to fix output parsing
5. **Failure Avoidance**: Adding layers of checks to prevent operations from failing

### 🎯 Trigger Questions
Ask these questions when debugging:
- "Am I trying to predict if an operation will succeed?"
- "Is this failure recoverable and informative?"
- "Would the failure tell me exactly what I'm trying to detect?"
- "What if I just tried the operation and handled the failure?"

## The Better Pattern

### Instead of Prediction:
```bash
# ❌ Complex prediction to avoid failure
if complex_parsing_of_tool_output | grep -q "ready_state"; then
  dependent_operation
else
  echo "Not ready"
fi
```

### Embrace Failure:
```bash
# ✅ Let the operation tell us when it's ready
for i in {1..N}; do
  if dependent_operation; then
    echo "Success!"
    exit 0
  else
    echo "Not ready, retrying in ${WAIT}s..."
    sleep ${WAIT}
  fi
done
```

## Common Applications

### Package/Dependency Management
- **Don't**: Parse `npm list`, `cargo info`, `pip show` to detect availability
- **Do**: Retry the install/publish operation directly

### Service Dependencies
- **Don't**: Build complex health checks and readiness probes
- **Do**: Retry the actual operation that needs the service

### File System Operations
- **Don't**: Check permissions, existence, locks separately
- **Do**: Try the operation and handle specific error types

### API/Network Operations
- **Don't**: Ping endpoints or check status pages
- **Do**: Retry the actual API call with appropriate backoff

## Implementation Guidelines

### Retry Pattern Template
```bash
operation_with_retry() {
  local max_attempts=${1:-10}
  local wait_time=${2:-10}
  local operation_name=${3:-"operation"}
  
  for i in $(seq 1 $max_attempts); do
    echo "Attempt $i: $operation_name"
    if your_actual_operation; then
      echo "$operation_name succeeded!"
      return 0
    else
      if [ $i -lt $max_attempts ]; then
        echo "$operation_name failed, retrying in ${wait_time}s..."
        sleep $wait_time
      fi
    fi
  done
  
  echo "ERROR: $operation_name failed after $max_attempts attempts"
  return 1
}
```

### Error Handling
- **Capture exit codes**: Use `$?` to distinguish different failure types
- **Log meaningfully**: Show what failed and why
- **Fail fast on permanent errors**: Don't retry authentication failures, syntax errors, etc.
- **Exponential backoff**: For network operations, increase wait time between retries

## Decision Framework

When encountering workflow/automation issues:

1. **Identify the core operation**: What actually needs to succeed?
2. **Check if failure is informative**: Does the failure tell you what's wrong?
3. **Assess recoverability**: Is this a temporary condition that might resolve?
4. **Consider retry**: Would retrying the operation be simpler than predicting success?

### When NOT to Apply
- **Permanent failures**: Authentication, permissions, syntax errors
- **Expensive operations**: Long-running processes, resource-intensive tasks
- **Side effects**: Operations that shouldn't be repeated (payments, data mutations)
- **Clear state requirements**: When you genuinely need to verify state before proceeding

## Real-World Example

**Problem**: Publishing dependent packages to a registry
**Anti-pattern**: Parse registry API to detect when dependency is available
**Better approach**: Retry publishing the dependent package directly

**Result**: 18 failed attempts at parsing vs. 1 successful retry implementation

## Key Insights

1. **Failures contain information**: Often exactly the information you're trying to detect
2. **Prediction is fragile**: Output formats change, APIs evolve, state detection breaks
3. **Operations know their requirements**: Let them fail with meaningful errors
4. **Simplicity wins**: Fewer moving parts, fewer failure modes
5. **Tools have built-in logic**: Use it instead of reimplementing it

## Prompt Integration

When debugging automation issues, always consider: **"Instead of trying to predict success, what if I just retry the operation and let it tell me when it's ready?"**

This single question can save hours of debugging complex parsing logic and lead to more robust, maintainable solutions.

---

*This guidance is based on a real debugging session where embracing failure led to a solution that worked on the first try, after 18 versions of trying to avoid the failure.*