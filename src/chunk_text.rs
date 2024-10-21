use unicode_segmentation::UnicodeSegmentation;

/// Returns indices of chunks using sliding window chunking, respecting grapheme clusters.
///
/// # Arguments
///
/// * `text` - The input string to be chunked.
/// * `chunk_size` - The desired size of each chunk in graphemes.
/// * `stride` - The number of graphemes to skip between chunks.
///
/// # Returns
///
/// Iterator of chunk indices
pub fn sliding_window_chunk_indices(
    text: &str,
    chunk_size: usize,
    stride: usize,
) -> impl Iterator<Item = (usize, usize)> + '_ {
    // assert that chunk_size is greater than 0
    assert!(chunk_size > 0);

    // Extra state for when end of text is reached
    let mut done = false;

    // Index of the last character in text
    let text_len = text.len();

    text.grapheme_indices(true)
        .step_by(stride)
        .flat_map(move |(from, _)| {
            let chunk = text[from..].grapheme_indices(true).skip(chunk_size).next();

            if done {
                return None;
            }

            match chunk {
                Some((to, _)) => Some((from, from + to)),
                None => {
                    done = true;
                    Some((from, text_len))
                }
            }
        })
}

/// Chunks a string using sliding window chunking, respecting grapheme clusters.
///
/// # Arguments
///
/// * `text` - The input string to be chunked.
/// * `chunk_size` - The desired size of each chunk in graphemes.
/// * `stride` - The number of graphemes to skip between chunks.
///
/// # Returns
///
/// Iterator of chunks
#[allow(dead_code)]
pub fn sliding_window_chunk(
    text: &str,
    chunk_size: usize,
    stride: usize,
) -> impl Iterator<Item = &str> {
    sliding_window_chunk_indices(text, chunk_size, stride).map(|(from, to)| &text[from..to])
}

#[cfg(test)]
mod tests {
    use crate::chunk_text::sliding_window_chunk;

    #[test]
    fn test_sliding_window_chunk() {
        let text = "This is a sample text for chunking.";
        let chunks: Vec<&str> = sliding_window_chunk(text, 10, 5).collect();
        assert_eq!(
            chunks,
            vec![
                "This is a ",
                "is a sampl",
                "sample tex",
                "e text for",
                "t for chun",
                " chunking."
            ]
        );
    }

    #[test]
    fn test_short_input() {
        let text = "Short";
        let chunks: Vec<&str> = sliding_window_chunk(text, 10, 5).collect();
        assert_eq!(chunks, vec!["Short"]);
    }

    #[test]
    fn test_no_overlap() {
        let text = "This is a test.";
        let chunks: Vec<&str> = sliding_window_chunk(text, 5, 5).collect();
        assert_eq!(chunks, vec!["This ", "is a ", "test."]);
    }

    #[test]
    fn test_full_overlap() {
        let text = "Overlapping";
        let chunks: Vec<&str> = sliding_window_chunk(text, 5, 1).collect();
        assert_eq!(
            chunks,
            vec!["Overl", "verla", "erlap", "rlapp", "lappi", "appin", "pping"]
        );
    }

    #[test]
    fn test_unicode() {
        let text = "こんにちは世界";
        let chunks: Vec<&str> = sliding_window_chunk(text, 3, 2).collect();
        assert_eq!(chunks, vec!["こんに", "にちは", "は世界"]);
    }

    #[test]
    fn test_emoji() {
        let text = "Hello 👨‍👩‍👧‍👦 World";
        let chunks: Vec<&str> = sliding_window_chunk(text, 7, 6).collect();
        assert_eq!(chunks, vec!["Hello 👨‍👩‍👧‍👦", "👨‍👩‍👧‍👦 World"]);
    }

    #[test]
    fn test_combining_characters() {
        let text = "e\u{301}e\u{301}e\u{301}"; // ééé
        let chunks: Vec<&str> = sliding_window_chunk(text, 2, 1).collect();
        assert_eq!(chunks, vec!["e\u{301}e\u{301}", "e\u{301}e\u{301}"]);
    }
}
