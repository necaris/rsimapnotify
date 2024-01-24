---
name: Improvement Plan
about: Use this template to propose new features, refactorings, or other technical improvements
title: "[DATE]: FEATURE NAME"
labels: proposal, needs review
---
# tl;dr

One sentence, or maybe two, explaining at a high level what's going on here. For example, 'Rewrite data pipeline in Common Lisp so we can stockpile parentheses'.

## What problem are you trying to solve?
Introduce the problem statement here. Can be well-defined or more fuzzy, e.g. 
  - we need to add a Boolean column `is_complex` to CDF to short-circuit triage
  - we need to invest more in paying back technical debt

Focus on the underlying problem that needs to be solved, as opposed to jumping to how you will solve it.

## Why should we solve it?
Why is this an important problem for us to solve?

Should we solve it now, or later?

_Provide specific data (quantitative or qualitative) wherever possible._

## How do you propose to solve it?
Add details of your proposed change. This can be high-level as pseudocode and sketches of the architecture, or as low-level as Protocol Buffer and interface definitions. Decide what the context of the proposal is, and write appropriately.

Be explicit about what you are optimising for in this proposed solution — for example, 'we need to hit this deadline, so we'll accept the tech debt this entails'.

## What other approaches did you consider?
In general, it's a red-flag if you haven't thought of a couple of different ways that you could approach the problem.

What does your proposed solution give you that these approaches don't? What are they optimising for that isn't appropriate in this case?

## What could go wrong?
What risks does your proposed change entail? How will you mitigate them?

How could this system fail, and what would be the impact if it did?

