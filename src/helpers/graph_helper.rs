#[macro_export]
macro_rules! graph_builder {
    ( $( [ $( $x:expr ),* ] ),*) => {
        vec![ $(vec![$($x),*]),* ]
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
        assert_eq!(vec![vec![()], vec![(),()]], graph_builder![[()], [(),()]]);
    }
}
