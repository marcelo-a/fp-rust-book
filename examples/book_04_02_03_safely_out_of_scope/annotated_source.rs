
fn calculate_length(<tspan data-hash="1">s</tspan>: &amp;String) -> usize {
    <tspan data-hash="1">s</tspan>.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.