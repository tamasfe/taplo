The `opt-level` setting controls the [`-C opt-level` flag](https://doc.rust-lang.org/rustc/codegen-options/index.html#opt-level) which controls the level
of optimization. Higher optimization levels may produce faster runtime code at
the expense of longer compiler times. Higher levels may also change and
rearrange the compiled code which may make it harder to use with a debugger.

It is recommended to experiment with different levels to find the right
balance for your project. There may be surprising results, such as level `3`
being slower than `2`, or the `"s"` and `"z"` levels not being necessarily
smaller. You may also want to reevaluate your settings over time as newer
versions of `rustc` changes optimization behavior.

See also [Profile Guided Optimization](https://doc.rust-lang.org/rustc/profile-guided-optimization.html) for more advanced optimization
techniques.