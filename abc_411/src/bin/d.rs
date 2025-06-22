use proconio::input;

const SERVER: usize = 0;

enum Query {
    FromServer(usize),
    AppendToPC(usize, String),
    ToServer(usize),
}

fn main() {
    input! {
        _: usize, q: usize,
    }

    let mut queries = (0..q)
        .map(|_| {
            input! { t: usize }
            match t {
                1 => {
                    input! { p: usize }
                    Query::FromServer(p)
                }
                2 => {
                    input! { p: usize, c: String }
                    Query::AppendToPC(p, c)
                }
                3 => {
                    input! { p: usize }
                    Query::ToServer(p)
                }
                _ => unreachable!(),
            }
        })
        .collect::<Vec<_>>();
    queries.reverse();

    let mut stack = vec![];

    let mut ptr = SERVER;

    for query in queries {
        match query {
            Query::FromServer(p) if ptr == p => {
                ptr = SERVER;
            }
            Query::ToServer(p) if ptr == SERVER => {
                ptr = p;
            }
            Query::AppendToPC(p, c) if ptr == p => {
                stack.push(c);
            }
            _ => {}
        }
    }

    stack.reverse();
    println!("{}", stack.join(""));
}
