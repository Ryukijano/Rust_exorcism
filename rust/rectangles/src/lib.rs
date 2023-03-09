
fn count_rectangles(diagram: &str) -> u32 {
    let mut count = 0;
    let lines: Vec<&str> = diagram.trim().split('\n').collect();

    // Iterate over all pairs of horizontal lines
    for i in 0..lines.len() {
        for j in (i+1)..lines.len() {
            let h1 = lines[i];
            let h2 = lines[j];

            // Find all pairs of vertical lines between the two horizontal lines
            for k in 0..h1.len() {
                for l in (k+1)..h1.len() {
                    let v1 = &h1[k..=k];
                    let v2 = &h2[k..=k];
                    let v3 = &h1[l..=l];
                    let v4 = &h2[l..=l];

                    // Check if the four points form a rectangle
                    if v1 == "+" && v2 == "+" && v3 == "+" && v4 == "+" {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

