/// Converts array of variable length array to vector of vector
#[macro_export]
macro_rules! graph_builder {
    (
        // outer array
        $(
            // inner array
            [
                $(
                    // each element is expression (could have been literal)
                    $x:expr
                // ,* -> one or more comma separated repetition
                ),*
                // allow trailing comma
                $(,)?
            ]
        // ,* -> one or more comma separated repetition
        ),*
        // allow trailing comma
        $(,)?
    ) => {
        // outer array converted to vector of comma separated vector
        vec![$(vec![
            // inner array expression are put here comma separated
            $($x),*
        ]),*]
    };
}
#[cfg(test)]
mod tests {

    #[test]
    fn empty_graph() {
        let _: Vec<Vec<()>> = graph_builder![];
    }

    #[test]
    fn multiple_graph() {
        assert_eq!(vec![vec![()], vec![(), ()]], graph_builder![[()], [(), ()]]);
    }

    #[test]
    fn multiple_graph_with_traling_comma() {
        assert_eq!(
            vec![vec![()], vec![(), ()]],
            graph_builder![[(),], [(), (),],]
        );
    }
}
