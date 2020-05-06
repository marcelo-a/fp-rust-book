
fn <tspan class="fn" data-hash="0" hash="3">calculate_length</tspan>(<tspan data-hash="5">s</tspan>: &amp;String) -> usize {
    <tspan data-hash="5">s</tspan>.<tspan class="fn" data-hash="0" hash="2">len()</tspan>
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.