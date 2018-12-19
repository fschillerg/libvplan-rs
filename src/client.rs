use crate::error::RequestError;
use crate::parser;
use crate::vplan;
use base64;
use chrono::Weekday;
use futures::{future, Future, Poll, Stream};
use http;
use hyper;
use hyper_tls;
use std::boxed::Box;

/// A client to retrieve a vplan.
///
/// # Example
/// ```rust,ignore
/// extern crate chrono;
/// extern crate futures;
/// extern crate libvplan;
/// extern crate tokio;
///
/// use chrono::Weekday;
/// use libvplan::Client;
/// use tokio::runtime::Runtime;
///
/// let client = Client::new("username", "password");
///
/// let future = client.get_vplan(Weekday::Mon);
///
/// let mut rt = match Runtime::new() {
///     Ok(rt) => rt,
///     Err(error) => panic!(error)
/// };
///
/// let result = rt.block_on(future);
///
/// println!("{:#?}", result);
/// ```
pub struct Client {
    client: hyper::Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector>>,
    url: String,
    authorization: Option<String>
}

/// A `Future` that will resolve to a vplan or an error during fetching it.
pub struct ResponseFuture {
    inner: Box<Future<Item = vplan::Vplan, Error = RequestError> + Send>
}

impl ResponseFuture {
    fn new(future: Box<Future<Item = vplan::Vplan, Error = RequestError> + Send>) -> Self {
        Self { inner: future }
    }
}

impl Future for ResponseFuture {
    type Item = vplan::Vplan;
    type Error = RequestError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.inner.poll()
    }
}

impl Client {
    /// Creates a new client for the given url.
    pub fn new(url: &str) -> Self {
        let connector = hyper_tls::HttpsConnector::new(4).unwrap();

        Self {
            client: hyper::Client::builder().keep_alive(false).build(connector),
            url: url.to_owned(),
            authorization: None
        }
    }

    /// Creates a new client via a preformatted HTTP Basic authentication string (base64 "username:password").
    pub fn with_auth(url: &str, authorization: &str) -> Self {
        let connector = hyper_tls::HttpsConnector::new(4).unwrap();

        Self {
            client: hyper::Client::builder().keep_alive(true).build(connector),
            url: url.to_owned(),
            authorization: Some(authorization.to_owned())
        }
    }

    /// Creates a new client with username and password for HTTP Basic authentication.
    pub fn with_credentials(url: &str, username: &str, password: &str) -> Self {
        Self::with_auth(
            url,
            format!(
                "Basic {}",
                base64::encode(format!("{}:{}", username, password).as_bytes())
            )
            .as_str()
        )
    }

    /// Retrieves the timetable change for the given day
    pub fn get(&self, day: Weekday) -> ResponseFuture {
        let day = match day {
            Weekday::Mon => Some("Mo"),
            Weekday::Tue => Some("Di"),
            Weekday::Wed => Some("Mi"),
            Weekday::Thu => Some("Do"),
            Weekday::Fri => Some("Fr"),
            _ => None
        };

        if day.is_none() {
            return ResponseFuture::new(Box::new(future::err(RequestError::InvalidDay)));
        }

        let day = day.unwrap();

        let url = format!("{}/tag_{}.xml", self.url, day);
        let uri = url.parse::<hyper::Uri>();

        if let Err(error) = uri {
            return ResponseFuture::new(Box::new(future::err(RequestError::URLParsingError(error))));
        }

        let uri = uri.unwrap();

        let request = match &self.authorization {
            Some(auth) => hyper::Request::builder()
                .method(http::Method::GET)
                .uri(uri)
                .header(http::header::AUTHORIZATION, auth.as_str())
                .body(hyper::Body::empty()),
            None => hyper::Request::builder()
                .method(http::Method::GET)
                .uri(uri)
                .body(hyper::Body::empty())
        };

        if let Err(error) = request {
            return ResponseFuture::new(Box::new(future::err(RequestError::Http(error))));
        }

        let request = request.unwrap();

        ResponseFuture::new(Box::new(
            self.client
                .request(request)
                .and_then(|response| {
                    let (parts, body) = response.into_parts();
                    body.concat2().map(|body| (parts, body))
                })
                .from_err()
                .and_then(move |(_, body)| {
                    let body = String::from_utf8(body.to_vec());

                    if let Err(error) = body {
                        return Err(RequestError::BodyParsingError(error));
                    }

                    let body = body.unwrap();

                    match parser::parse(body.as_ref()) {
                        Ok(vplan) => Ok(vplan),
                        Err(error) => Err(RequestError::XMLParsingError(error))
                    }
                })
        ))
    }
}
