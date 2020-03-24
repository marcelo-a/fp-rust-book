
fn <tspan data-hash="3">calculate_length</tspan>(<tspan data-hash="4">s</tspan>: &amp;String) -> usize {
    <tspan data-hash="4">s</tspan>.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.