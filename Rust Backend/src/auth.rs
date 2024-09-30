// use warp::{
//     filters::header::headers_cloned,
//     http::header::{HeaderMap, HeaderValue, AUTHORIZATION},
//     reject, Filter, Rejection,
// };

// pub fn with_auth(role: Role) -> impl Filter<Extract = (String,), Error = Rejection> + Clone {
//     headers_cloned()
//         .map(move |headers: HeaderMap<HeaderValue>| (role.clone(), headers))
//         .and_then(authorize)
// }
