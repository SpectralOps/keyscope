<p align="center">
<br/>
<br/>
<br/>
   <img src="media/ks.png" width="1019"/>
<br/>
<br/>
</p>
<p align="center">
<b>:white_check_mark: Automate your key and secret validation workflows</b>
<br/>
<b>:cowboy_hat_face: Over 30 different providers</b>
<br/>
<b>:robot: Export to JSON, audit via CSV</b>
<br/>
<hr/>
</p>

<p align="center">
<img src="https://github.com/spectralops/keyscope/actions/workflows/build.yml/badge.svg"/>

</p>

# :key: Keyscope

Keyscope is a key and secret workflow (validation, invalidation, etc.) tool built in Rust, powered by `service_policy_kit`.

Current workflows supported:

* Validation

# 🦀 Why Rust?

* With Rust, _"If it compiles, it works."_ and also, it compiles to many platforms.
* Rust is _fast_, has no _VM_, or unnecessary cruft (typically 5-8mb binaries with LOTS of code and libraries).
* Multi purpose, safe, and generalistic - makes for healthy and expressive mission critical code. Adding code or abstraction doesn't increase bloat, doesn't hurt performance, doesn't increase chance for bugs in a radical way (less edge cases).
* Amazing package manager: `Cargo`. Productive installing and running of tasks and examples.
* Rust is getting headlines in the security community as the go-to language for security tools. Equally interesting is offensive security + Rust [here](https://github.com/trickster0/OffensiveRust) and [here](https://github.com/skerkour/black-hat-rust).

# :rocket: Quick Start

Grab a release from [releases](https://github.com/spectralops/keyscope/releases), or install via Homebrew:

```
brew tap spectralops/tap && brew install keyscope
```

## Using keyscope

You can try out validating a key for a provider, say, Github (assuming the key  is in the `GITHUB_TOKEN` environment variable):

```
$ keyscope validate github $GITHUB_TOKEN
```

You can see which other providers are supported by running:

```
$ keyscope validate --providers

  .
  :
  .

twilio:validation
keyscope validate twilio -p p1 p2

twitter:validation
keyscope validate twitter -p p1

zendesk:validation
keyscope validate zendesk -p p1 p1

Total 33 providers available.
$
```

And what parameters are required for a certain provider by running (say, `stripe`):

```
$ keyscope validate stripe --requirements

provider stripe requires:
 - param: p1
   desc: stripe key
$
```

Finally the general structure of the `validate` command is:

```
$ keyscope validate PROVIDER -p PARAM1 PARAM2 .. PARAM_N
```

# :white_check_mark: Validation: key should be active


You can validate a specific provider like so:

```
$ keyscope validate twilio -p $TWILIO_KEY
```

With the general pattern of:

```
$ keyscope validate PROVIDER -p PARAM1 PARAM2 ...
```

The number of keys/params would change based on authentication type:

* `Bearer` - usually just a single key (token)
* `Basic Auth` - usually 2 keys: user, password
* `OAuth` - usually 2 keys: client_id, client_secret
* And others.

Each provider in Keyscope will tell you what it requires using `requirements`:

```
$ keyscope validate twilio --requirements
```

You'll get a report:

```
$ keyscope --verbose validate stripe -p $STRIPE_KEY

✔ stripe:validation: ok 766ms

Ran 1 interactions with 1 checks in 766ms

Success: 1
Failure: 0
  Error: 0
Skipped: 0
```

And an executable exit code that reflects success/failure.

You can use the `--verbose` flag to see API responses:

```
$ keyscope --verbose validate stripe -p $STRIPE_KEY

✗ stripe:validation: failed 413ms
      status_code: 200 != 401 Unauthorized

Ran 1 interactions with 1 checks in 413ms

Success: 0
Failure: 1
  Error: 0
Skipped: 0
```

In this case the exit code is `1` (failure).

# :x: Validation: key should be inactive

When you are validating keys that are supposed to be inactive, you can use the `flip` flag. In this mode, a failed API access is a good thing, and the exit code will reflect that.

```
$ keyscope --flip validate stripe -p $STRIPE_KEY

✔ stripe:validation: ok 766ms

Ran 1 interactions with 1 checks in 766ms
```

In this case, the key is active - _which is bad for us_. Using `--flip`, the exit code will be `1` (failure).

# :runner: Setting up a validation job

## Audit active keys

You can set up a CI job (or other form of scheduled job you like) to perform an audit, by reading all parameters from a dedicated CSV file like so:

```
$ keyscope validate --csv-in report.csv
```

The format of the CSV that you need to prepare should include a header line and look like this:

```
provider,key1,key2,key3
twilio,tw-key1,,,
```

You can specify as many key columns as you like, as long as you provide an _empty_ value for providers which don't have that many keys, and all rows contain the same amount of cells.




## Audit inactive keys

If you have a dump of keys from your vault that are stale have expiry and should have been rotated, you want to test that they are all stale:

```
$ keyscope --flip validate --csv-in my-key-audit.csv
```


# :link: Supported providers

We're always adding [new providers](src/defs.yaml), keep a link to this list or watch this repo to get updated.

We use our `service_policy_toolkit` library to specify interactions with services and their policies, if you find a service [not in this list](src/defs.yaml) feel free to open an issue or contribute back.


<!-- providers -->
| provider              | actions    | params                                                                                                 |
| --------------------- | ---------- | ------------------------------------------------------------------------------------------------------ |
| hookbin               | validation | `hookbin_1` - hookbin ID (https://hookb.in)<br/>`hookbin_2` - fake key to put as a query param         |
| covalenthq            | validation | `covalenthq_1` - covalent token                                                                        |
| asana                 | validation | `asana_1` - asana token                                                                                |
| bitly                 | validation | `bitly_1` - bit.ly token                                                                               |
| localytics            | validation | `localytics_1` - localytics user<br/>`localytics_2` - localytics key                                   |
| algolia               | validation | `algolia_1` - algolia application ID<br/>`algolia_2` - algolia index<br/>`algolia_3` - algolia API key |
| branchio              | validation | `branchio_1` - branch.io key<br/>`branchio_2` - branch.io secret                                       |
| browserstack          | validation | `browserstack_1` - browserstack key<br/>`browserstack_2` - browserstack secret                         |
| buildkite             | validation | `buildkite_1` - buildkite token                                                                        |
| datadog               | validation | `datadog_1` - datadog API key                                                                          |
| github                | validation | `github_1` - github token                                                                              |
| dropbox               | validation | `dropbox_1` - dropbox token                                                                            |
| gitlab                | validation | `gitlab_1` - gitlab token                                                                              |
| heroku                | validation | `heroku_1` - heroku token                                                                              |
| mailchimp             | validation | `mailchimp_1` - mailchimp datacenter ID<br/>`mailchimp_2` - mailchimp key                              |
| mailgun               | validation | `mailgun_1` - mailgun key                                                                              |
| pagerduty             | validation | `pagerduty_1` - pagerduty token                                                                        |
| circleci              | validation | `circleci_1` - circleci key                                                                            |
| facebook-access-token | validation | `facebook-access-token_1` - facebook token                                                             |
| salesforce            | validation | `salesforce_1` - salesforce instance name<br/>`salesforce_2` - salesforce token                        |
| jumpcloud             | validation | `jumpcloud_1` - jumpcloud key                                                                          |
| saucelabs-us          | validation | `saucelabs-us_1` - saucelabs user<br/>`saucelabs-us_2` - saucelabs key                                 |
| saucelabs-eu          | validation | `saucelabs-eu_1` - saucelabs user<br/>`saucelabs-eu_2` - saucelabs key                                 |
| sendgrid              | validation | `sendgrid_1` - sendgrid key                                                                            |
| slack                 | validation | `slack_1` - slack key                                                                                  |
| slack-webhook         | validation | `slack-webhook_1` - slack webhook                                                                      |
| stripe                | validation | `stripe_1` - stripe key                                                                                |
| travisci              | validation | `travisci_1` - travisci domain, choose 'org' or 'com'<br/>`travisci_2` - travisci key                  |
| twilio                | validation | `twilio_1` - twilio account sid<br/>`twilio_2` - twilio token                                          |
| twitter               | validation | `twitter_1` - twitter API token                                                                        |
| zendesk               | validation | `zendesk_1` - zendesk domain<br/>`zendesk_2` - zendesk key                                             |
| firebase              | validation | `firebase_1` - firebase API key<br/>`firebase_2` - firebase ID token                                   |
| aws                   | validation | `aws_1` - AWS ID<br/>`aws_2` - AWS secret                                                              |
<!-- /providers -->






# :cake: Adding your own providers

You can specify a custom definitions file (here [is an example](custom-defs.example.yaml)):

```
$ keyscope -f your-definitions.yaml validate --list
```

Which is suitable for adding your own internal services, key issuing policies, and infrastructure.

You can also use custom definitions to test out new providers that you'd like to contribute back to keyscope :smile:

## Basics of a definition

All definitions represent an interaction. A request being made, and a policy that's being checked against it.

```yaml
providers:
  hookbin:
    validation:
      #
      # the request part
      #
      request:
        params:
        - name: hookbin_1
          desc: hookbin ID (https://hookb.in)
        - name: hookbin_2
          desc: fake key to put as a query param
        id: "postbin:validation"
        desc: "Postbin: valid key"
        # variable interpolation is good for all fields
        uri: "https://hookb.in/{{hookbin_1}}?q={{hookbin_2}}"
        method: post
        # you can also use headers, body, form, basic_auth and more (see defs.yaml)

      # 
      # the policy part: all values are actually regular expressions and are matched against service response
      #
      response:
        status_code: "200"
        body: ok
```

When in doubt, you can check keyscope's own [defs.yaml](src/defs.yaml) for real examples of how to use this infrastructure.





# Thanks

To all [Contributors](https://github.com/spectralops/keyscope/graphs/contributors) - you make this happen, thanks!


# Copyright

Copyright (c) 2021 [@jondot](http://twitter.com/jondot). See [LICENSE](LICENSE.txt) for further details.
