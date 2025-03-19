use std::collections::HashMap;

/**
 What is an Algorithm?
 An Algorithm is a set of instructions or steps needed to accomplish a task.

How do you improve at solving a problem?
 1. Device a plan for solving problems.
    - Understand the problem
      1. Can I restate the problem in my own words?
      2. What are the inputs that go into the problem?
      3. What are the outputs that should come from the solution to the problem?
      4. Can the outputs be determined from the inputs?
      5. How should I label the important pieces of data that are a part of the problem?

    - Explore Concrete Examples
    - Break It Down
    - Solve/Simplify
      1. Find the core difficulty in what you're trying to do.
      2. Temporarily ignore that difficulty.
      3. Write a simplified solution.
      4. Then incorporate that difficulty back in.
    - Look Back and Refactor

 2. Master common problem solving patterns.

    1. Frequency Counter Pattern
    - Uses objects or sets to collect values/frequencies
    - Useful for comparing pieces of data or frequencies of occurrences

    2. Multiple Pointers Pattern
    - Creating pointers that correspond to index positions
    - Moving toward beginning, end, or middle based on conditions

    3. Sliding Window Pattern
    - Creating a window that either expands or contracts
    - Useful for keeping track of a subset of data in an array/string

    4. Divide and Conquer Pattern
    - Dividing data into smaller chunks
    - Usually involves array or strings
    - Decreases time complexity

    5. Recursion Pattern
    - Function that calls itself
    - Useful for tasks that have repeated subtasks

    6. Dynamic Programming Pattern
    - Breaking down complex problems into simpler subproblems
    - Storing results for future use (memoization)

    7. Greedy Pattern
    - Making locally optimal choices at each step
    - Hoping to find global optimum

    8. Backtracking Pattern
    - Building candidates to the solution incrementally
    - Abandoning candidates ("backtracking") if they don't satisfy the constraints

    9. Two Heaps Pattern
    - Using two heaps to track median of a set of numbers
    - One min heap and one max heap

    10. Binary Search Pattern
    - Searching a sorted array by repeatedly dividing the search interval in half

    11. State Machine Pattern
    - Modeling solutions using finite state machines
    - Useful for parsing and processing sequential data

    12. Graph Traversal Patterns
    - Depth-First Search (DFS)
    - Breadth-First Search (BFS)

    13. Tree Patterns
    - Various traversal methods
    - Construction and manipulation techniques

*/

// Workshop ->

/**
- Problem: Write a function which takes two numbers and returns their sum.

- Understand the problem
  1. Can I restate the problem in my own words?
     -> Implement addition
  2. What are the inputs that go into the problem?
     -> ints?
     -> floats?
     -> what about string for large numbers?
  3. What are the outputs that should come from the solution to the problem?
     -> ints?
     -> floats?
     -> string?
  4. Can the outputs be determined from the inputs?
  5. How should I label the important pieces of data that are a part of the problem?
*/

pub fn char_count(value: &str) -> HashMap<char, u32> {
    let mut map = HashMap::new();

    for char in value.to_lowercase().chars() {
        if char.is_alphanumeric() {
            let value = map.get(&char);

            if value.is_some() {
                let item = value.unwrap();
                map.insert(char, item + 1);
            } else {
                map.insert(char, 1);
            }
        }
    }

    map
}

pub fn char_count_lite(value: &str) -> HashMap<char, u32> {
    let mut map = HashMap::new();

    for char in value.to_lowercase().chars() {
        if char.is_alphanumeric() {
            *map.entry(char).or_insert(0) += 1;
        }
    }

    map
}
