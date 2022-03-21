// output time!
// {
//   "host": "imap.gmail.com",
//   "port": 993,
//   "tls": true,
//   "tlsOptions": {
//     "rejectUnauthorized": true
//   },
//   "username": "rchowdhury@coiled.io",
// "passwordCmd": "pass coiled/rchowdhury@coiled.io",
//   "xoauth2": false,
//   "onNewMailPost": "emacsclient  -e '(mu4e-update-index)'",
//   "wait": 20,
//   "boxes": [
//     "INBOX"
//   ]
// }

use super::*;
use serde_json::json;

pub fn to_goimapnotify(c: &Channel) -> serde_json::Value {
    json!({
        "host": "imap.gmail.com",
        "port": 993,
        "tls": true,
        "tlsOptions": {
            "rejectUnauthorized": true
        },
        "username": "rchowdhury@coiled.io",
        "passwordCmd": "pass coiled/rchowdhury@coiled.io",
        "xoauth2": false,
        "onNewMailPost": "emacsclient  -e '(mu4e-update-index)'",
        "wait": 20,
        "boxes": [
            "INBOX"
        ]
    })
}
