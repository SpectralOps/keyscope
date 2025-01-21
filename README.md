<p align="center">
<br/>
<br/>
<br/>
   <img src="media/ks.png" width="420"/>
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
<img src="media/screen.png" width="1019"/>
</p>

# :key: Keyscope <img src="https://github.com/spectralops/keyscope/actions/workflows/build.yml/badge.svg"/>

Keyscope is a key and secret workflow (validation, invalidation, etc.) tool built in Rust, powered by [`service_policy_kit`](https://github.com/spectralops/service-policy-kit).

Current workflows supported:

- Validation

# 🦀 Why Rust?

- With Rust, _"If it compiles, it works."_ and also, it compiles to many platforms.
- Rust is _fast_, has no _VM_, or unnecessary cruft (typically 5-8mb binaries with LOTS of code and libraries).
- Multi purpose, safe, and generalistic - makes for healthy and expressive [mission critical code](https://www.youtube.com/watch?v=ylOpCXI2EMM). Adding code or abstraction doesn't increase bloat, doesn't hurt performance, doesn't increase chance for bugs in a radical way (less edge cases).
- Amazing package manager: `Cargo`. Productive installing and running of tasks and examples.
- Rust is getting headlines in the security community as the go-to language for security tools. Equally interesting is offensive security + Rust [here](https://github.com/trickster0/OffensiveRust) and [here](https://github.com/skerkour/black-hat-rust).

# :rocket: Quick Start

Grab a release from [releases](https://github.com/spectralops/keyscope/releases), or install via Homebrew:

```
brew tap spectralops/tap && brew install keyscope
```

## Using keyscope

You can try out validating a key for a provider, say, Github (assuming the key is in the `GITHUB_TOKEN` environment variable):

```
$ keyscope validate github $GITHUB_TOKEN
```

You can see which other providers are supported by running:

```
$ keyscope validate --list

  .
  :
  .

twilio:validation
keyscope validate twilio -p twilio_1 -p twilio_2

twitter:validation
keyscope validate twitter -p twitter_1

zendesk:validation
keyscope validate zendesk -p zendesk_1 -p zendesk_2

Total 44 providers available.
$
```

And what parameters are required for a certain provider by running (say, `stripe`):

```
$ keyscope requirements stripe

provider stripe requires:
 - param: p1
   desc: stripe key
$
```

Finally the general structure of the `validate` command is:

```
$ keyscope validate PROVIDER -p PARAM1 -p PARAM2 .. -p PARAM_N
```

# :white_check_mark: Validation: key should be active

You can validate a specific provider like so:

```
$ keyscope validate twilio -p $TWILIO_KEY
```

With the general pattern of:

```
$ keyscope validate PROVIDER -p PARAM1 -p PARAM2 ...
```

The number of keys/params would change based on authentication type:

- `Bearer` - usually just a single key (token)
- `Basic Auth` - usually 2 keys: user, password
- `OAuth` - usually 2 keys: client_id, client_secret
- And others.

Each provider in Keyscope will tell you what it requires using `requirements`:

```
$ keyscope requirements twilio
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

We use our [`service_policy_kit`](https://github.com/spectralops/service-policy-kit) library to specify interactions with services and their policies, if you find a service [not in this list](src/defs.yaml) feel free to open an issue or contribute back.

<!-- providers -->
<table>
<tr><td>

**tester**<br/>Tester: valid key

</td>
<td>

validation

</td>
<td>

`tester_1` - hookbin ID (https://hookb.in)<br/>`tester_2` - fake key to put as a query param

</td>
</tr>
<tr>
<td colspan="3">

```
keyscope validate tester -p TESTER_1 -p TESTER_2
```

</td></tr>
<tr><td>

**infura**<br/>infura API key

</td>
<td>

validation

</td>
<td>

`infura_1` - infura API key

</td>
</tr>
<tr>
<td colspan="3">

```
keyscope validate infura -p INFURA_1
```

</td></tr>
<tr><td>

**covalenthq**<br/>Covalent: valid key

</td>
<td>

validation

</td>
<td>

`covalenthq_1` - covalent token

</td>
</tr>
<tr>
<td colspan="3">

```
keyscope validate covalenthq -p COVALENTHQ_1
```

</td></tr>
<tr><td>

**asana**<br/>Asana: valid token

</td>
<td>

validation

</td>
<td>

`asana_1` - asana token

</td>
</tr>
<tr>
<td colspan="3">

```
keyscope validate asana -p ASANA_1
```

</td></tr>
<tr><td>

**bitly**<br/>Bit.ly: valid access token

</td>
<td>

validation

</td>
<td>

`bitly_1` - bit.ly token

</td>
</tr>
<tr>
<td colspan="3">

```
keyscope validate bitly -p BITLY_1
```

</td></tr>
<tr><td>

**ipstack**<br/>ipstack access key

</td>
<td>

validation

</td>
<td>

`ipstack_1` - ipstack access key

</td>
</tr>
<tr>
<td colspan="3">

```
keyscope validate ipstack -p IPSTACK_1
```

</td></tr>
<tr><td>

**localytics**<br/>Localytics: valid API credentials

</td>
<td>

validation

</td>
<td>

`localytics_1` - localytics user<br/>`localytics_2` - localytics key

</td>
</tr>
<tr>
<td colspan="3">

```
keyscope validate localytics -p LOCALYTICS_1 -p LOCALYTICS_2
```

</td></tr>
<tr><td>

**algolia**<br/>Algolia: valid API credentials

</td>
<td>

validation

</td>
<td>

`algolia_1` - algolia application ID<br/>`algolia_2` - algolia index<br/>`algolia_3` - algolia API key

</td>
</tr>
<tr>
<td colspan="3">

```
keyscope validate algolia -p ALGOLIA_1 -p ALGOLIA_2 -p ALGOLIA_3
```

</td></tr>
<tr><td>

**branchio**<br/>branch.io: valid API credentials

</td>
<td>

validation

</td>
<td>

`branchio_1` - branch.io key<br/>`branchio_2` - branch.io secret

</td>
</tr>
<tr>
<td colspan="3">

```
keyscope validate branchio -p BRANCHIO_1 -p BRANCHIO_2
```

</td></tr>
<tr><td>

**browserstack**<br/>browserstack: valid API credentials

</td>
<td>

validation

</td>
<td>

`browserstack_1` - browserstack key<br/>`browserstack_2` - browserstack secret

</td>
</tr>
<tr>
<td colspan="3">

```
keyscope validate browserstack -p BROWSERSTACK_1 -p BROWSERSTACK_2
```

</td></tr>
<tr><td>

**buildkite**<br/>Buildkite: valid token

</td>
<td>

validation

</td>
<td>

`buildkite_1` - buildkite token

</td>
</tr>
<tr>
<td colspan="3">

```
keyscope validate buildkite -p BUILDKITE_1
```

</td></tr>
<tr><td>

**datadog**<br/>datadog: valid API credentials

</td>
<td>

validation

</td>
<td>

`datadog_1` - datadog API key

</td>
</tr>
<tr>
<td colspan="3">

```
keyscope validate datadog -p DATADOG_1
```

</td></tr>
<tr><td>

**github**<br/>github: valid API credentials

</td>
<td>

validation

</td>
<td>

`github_1` - github token

</td>
</tr>
<tr>
<td colspan="3">

```
keyscope validate github -p GITHUB_1
```

</td></tr>
<tr><td>

**github-ent**<br/>Github Enterprise: valid API token

</td>
<td>

validation

</td>
<td>

`github-ent_1` - github enterprise instance (without http)<br/>`github-ent_2` - github token

</td>
</tr>
<tr>
<td colspan="3">

```
keyscope validate github-ent -p GITHUB-ENT_1 -p GITHUB-ENT_2
```

</td></tr>
<tr><td>

**dropbox**<br/>dropbox: valid API credentials

</td>
<td>

validation

</td>
<td>

`dropbox_1` - dropbox token

</td>
</tr>
<tr>
<td colspan="3">

```
keyscope validate dropbox -p DROPBOX_1
```

</td></tr>
<tr><td>

**gitlab**<br/>gitlab: valid API credentials

</td>
<td>

validation

</td>
<td>

`gitlab_1` - gitlab token

</td>
</tr>
<tr>
<td colspan="3">

```
keyscope validate gitlab -p GITLAB_1
```

</td></tr>
<tr><td>

**heroku**<br/>heroku: valid API credentials

</td>
<td>

validation

</td>
<td>

`heroku_1` - heroku token

</td>
</tr>
<tr>
<td colspan="3">

```
keyscope validate heroku -p HEROKU_1
```

</td></tr>
<tr><td>

**mailchimp**<br/>mailchimp: valid API credentials

</td>
<td>

validation

</td>
<td>

`mailchimp_1` - mailchimp datacenter ID<br/>`mailchimp_2` - mailchimp key

</td>
</tr>
<tr>
<td colspan="3">

```
keyscope validate mailchimp -p MAILCHIMP_1 -p MAILCHIMP_2
```

</td></tr>
<tr><td>

**mailgun**<br/>mailgun: valid API credentials

</td>
<td>

validation

</td>
<td>

`mailgun_1` - mailgun key

</td>
</tr>
<tr>
<td colspan="3">

```
keyscope validate mailgun -p MAILGUN_1
```

</td></tr>
<tr><td>

**pagerduty**<br/>pagerduty: valid API credentials

</td>
<td>

validation

</td>
<td>

`pagerduty_1` - pagerduty token

</td>
</tr>
<tr>
<td colspan="3">

```
keyscope validate pagerduty -p PAGERDUTY_1
```

</td></tr>
<tr><td>

**circleci**<br/>circleci: valid API credentials

</td>
<td>

validation

</td>
<td>

`circleci_1` - circleci key

</td>
</tr>
<tr>
<td colspan="3">

```
keyscope validate circleci -p CIRCLECI_1
```

</td></tr>
<tr><td>

**facebook-access-token**<br/>facebook: valid API token

</td>
<td>

validation

</td>
<td>

`facebook-access-token_1` - facebook token

</td>
</tr>
<tr>
<td colspan="3">

```
keyscope validate facebook-access-token -p FACEBOOK-ACCESS-TOKEN_1
```

</td></tr>
<tr><td>

**salesforce**<br/>salesforce: valid API credentials

</td>
<td>

validation

</td>
<td>

`salesforce_1` - salesforce instance name<br/>`salesforce_2` - salesforce token

</td>
</tr>
<tr>
<td colspan="3">

```
keyscope validate salesforce -p SALESFORCE_1 -p SALESFORCE_2
```

</td></tr>
<tr><td>

**jumpcloud**<br/>jumpcloud: valid API credentials

</td>
<td>

validation

</td>
<td>

`jumpcloud_1` - jumpcloud key

</td>
</tr>
<tr>
<td colspan="3">

```
keyscope validate jumpcloud -p JUMPCLOUD_1
```

</td></tr>
<tr><td>

**saucelabs-us**<br/>saucelabs-us: valid API credentials

</td>
<td>

validation

</td>
<td>

`saucelabs-us_1` - saucelabs user<br/>`saucelabs-us_2` - saucelabs key

</td>
</tr>
<tr>
<td colspan="3">

```
keyscope validate saucelabs-us -p SAUCELABS-US_1 -p SAUCELABS-US_2
```

</td></tr>
<tr><td>

**saucelabs-eu**<br/>saucelabs-eu: valid API credentials

</td>
<td>

validation

</td>
<td>

`saucelabs-eu_1` - saucelabs user<br/>`saucelabs-eu_2` - saucelabs key

</td>
</tr>
<tr>
<td colspan="3">

```
keyscope validate saucelabs-eu -p SAUCELABS-EU_1 -p SAUCELABS-EU_2
```

</td></tr>
<tr><td>

**sendgrid**<br/>sendgrid: valid API credentials

</td>
<td>

validation

</td>
<td>

`sendgrid_1` - sendgrid key

</td>
</tr>
<tr>
<td colspan="3">

```
keyscope validate sendgrid -p SENDGRID_1
```

</td></tr>
<tr><td>

**slack**<br/>slack: valid API credentials

</td>
<td>

validation

</td>
<td>

`slack_1` - slack key

</td>
</tr>
<tr>
<td colspan="3">

```
keyscope validate slack -p SLACK_1
```

</td></tr>
<tr><td>

**slack-webhook**<br/>slack-webook: valid API credentials

</td>
<td>

validation

</td>
<td>

`slack-webhook_1` - slack webhook

</td>
</tr>
<tr>
<td colspan="3">

```
keyscope validate slack-webhook -p SLACK-WEBHOOK_1
```

</td></tr>
<tr><td>

**stripe**<br/>stripe: valid API credentials

</td>
<td>

validation

</td>
<td>

`stripe_1` - stripe key

</td>
</tr>
<tr>
<td colspan="3">

```
keyscope validate stripe -p STRIPE_1
```

</td></tr>
<tr><td>

**travisci**<br/>travisci: valid API credentials

</td>
<td>

validation

</td>
<td>

`travisci_1` - travisci domain, choose 'org' or 'com'<br/>`travisci_2` - travisci key

</td>
</tr>
<tr>
<td colspan="3">

```
keyscope validate travisci -p TRAVISCI_1 -p TRAVISCI_2
```

</td></tr>
<tr><td>

**twilio**<br/>twilio: valid API credentials

</td>
<td>

validation

</td>
<td>

`twilio_1` - twilio account sid<br/>`twilio_2` - twilio token

</td>
</tr>
<tr>
<td colspan="3">

```
keyscope validate twilio -p TWILIO_1 -p TWILIO_2
```

</td></tr>
<tr><td>

**twitter**<br/>twitter: valid API credentials

</td>
<td>

validation

</td>
<td>

`twitter_1` - twitter API token

</td>
</tr>
<tr>
<td colspan="3">

```
keyscope validate twitter -p TWITTER_1
```

</td></tr>
<tr><td>

**zendesk**<br/>zendesk: valid API credentials

</td>
<td>

validation

</td>
<td>

`zendesk_1` - zendesk domain<br/>`zendesk_2` - zendesk key

</td>
</tr>
<tr>
<td colspan="3">

```
keyscope validate zendesk -p ZENDESK_1 -p ZENDESK_2
```

</td></tr>
<tr><td>

**firebase**<br/>firebase: valid API credentials

</td>
<td>

validation

</td>
<td>

`firebase_1` - firebase API key<br/>`firebase_2` - firebase ID token

</td>
</tr>
<tr>
<td colspan="3">

```
keyscope validate firebase -p FIREBASE_1 -p FIREBASE_2
```

</td></tr>
<tr><td>

**aws**<br/>aws: valid API credentials

</td>
<td>

validation

</td>
<td>

`aws_1` - AWS ID<br/>`aws_2` - AWS secret

</td>
</tr>
<tr>
<td colspan="3">

```
keyscope validate aws -p AWS_1 -p AWS_2
```

</td></tr>
<tr><td>

**elastic-apm-secret**<br/>Elastic APM: secret key validation

</td>
<td>

validation

</td>
<td>

`elastic-apm-secret_1` - Elastic APM host address and port, including 'http/s' part<br/>`elastic-apm-secret_2` - Elastic APM secret

</td>
</tr>
<tr>
<td colspan="3">

```
keyscope validate elastic-apm-secret -p ELASTIC-APM-SECRET_1 -p ELASTIC-APM-SECRET_2
```

</td></tr>
<tr><td>

**artifactory**<br/>Artifactory: token validation

</td>
<td>

validation

</td>
<td>

`artifactory_1` - Artifactory host (including http(s) part)<br/>`artifactory_2` - Artifactory token

</td>
</tr>
<tr>
<td colspan="3">

```
keyscope validate artifactory -p ARTIFACTORY_1 -p ARTIFACTORY_2
```

</td></tr>
<tr><td>

**ibm-cos**<br/>IBM: cloud object storage key validation (HMAC)

</td>
<td>

validation

</td>
<td>

`ibm-cos_1` - IBM HMAC ID<br/>`ibm-cos_2` - IBM HMAC secret

</td>
</tr>
<tr>
<td colspan="3">

```
keyscope validate ibm-cos -p IBM-COS_1 -p IBM-COS_2
```

</td></tr>
<tr><td>

**ibm-iam**<br/>IBM: cloud key validation (IAM)

</td>
<td>

validation

</td>
<td>

`ibm-iam_1` - IBM cloud key

</td>
</tr>
<tr>
<td colspan="3">

```
keyscope validate ibm-iam -p IBM-IAM_1
```

</td></tr>
<tr><td>

**ibm-cloudant**<br/>IBM: cloudant key validation (legacy)

</td>
<td>

validation

</td>
<td>

`ibm-cloudant_1` - IBM cloudant hostname<br/>`ibm-cloudant_2` - IBM cloudant user<br/>`ibm-cloudant_3` - IBM cloudant key

</td>
</tr>
<tr>
<td colspan="3">

```
keyscope validate ibm-cloudant -p IBM-CLOUDANT_1 -p IBM-CLOUDANT_2 -p IBM-CLOUDANT_3
```

</td></tr>
<tr><td>

**softlayer**<br/>Softlayer: validate credentials

</td>
<td>

validation

</td>
<td>

`softlayer_1` - Softlayer hostname<br/>`softlayer_2` - Softlayer token

</td>
</tr>
<tr>
<td colspan="3">

```
keyscope validate softlayer -p SOFTLAYER_1 -p SOFTLAYER_2
```

</td></tr>
<tr><td>

**square**<br/>Square: valid token

</td>
<td>

validation

</td>
<td>

`square_1` - Square token

</td>
</tr>
<tr>
<td colspan="3">

```
keyscope validate square -p SQUARE_1
```

</td></tr>
<tr><td>

**telegram-bot**<br/>telegram-bot: valid bot token

</td>
<td>

validation

</td>
<td>

`telegram-bot_1` - bot key

</td>
</tr>
<tr>
<td colspan="3">

```
keyscope validate telegram-bot -p TELEGRAM-BOT_1
```

</td></tr>
<tr><td>

**bingmaps**<br/>Bing Maps API: valid access token

</td>
<td>

validation

</td>
<td>

`bingmaps_1` - Bing Maps token

</td>
</tr>
<tr>
<td colspan="3">

```
keyscope validate bingmaps -p BINGMAPS_1
```

</td></tr>
<tr><td>

**buttercms**<br/>ButterCMS: valid bot token

</td>
<td>

validation

</td>
<td>

`buttercms_1` - ButterCMS API key

</td>
</tr>
<tr>
<td colspan="3">

```
keyscope validate buttercms -p BUTTERCMS_1
```

</td></tr>
<tr><td>

**wakatime**<br/>wakatime: valid api token

</td>
<td>

validation

</td>
<td>

`wakatime_1` - WakeTime API key

</td>
</tr>
<tr>
<td colspan="3">

```
keyscope validate wakatime -p WAKATIME_1
```

</td></tr>
<tr><td>

**calendly**<br/>calendly: valid API credentials

</td>
<td>

validation

</td>
<td>

`calendly_1` - calendly API key

</td>
</tr>
<tr>
<td colspan="3">

```
keyscope validate calendly -p CALENDLY_1
```

</td></tr>
<tr><td>

**shodan**<br/>shodan: valid api token

</td>
<td>

validation

</td>
<td>

`shodan_1` - Shodan API key

</td>
</tr>
<tr>
<td colspan="3">

```
keyscope validate shodan -p SHODAN_1
```

</td></tr>
<tr><td>

**opsgenie**<br/>opsgenie: valid api token

</td>
<td>

validation

</td>
<td>

`opsgenie_1` - opsgenie API key

</td>
</tr>
<tr>
<td colspan="3">

```
keyscope validate opsgenie -p OPSGENIE_1
```

</td></tr>
<tr><td>

**pendo**<br/>pendo: valid api token

</td>
<td>

validation

</td>
<td>

`pendo_1` - pendo API key

</td>
</tr>
<tr>
<td colspan="3">

```
keyscope validate pendo -p PENDO_1
```

</td></tr>
<tr><td>

**hubspot**<br/>hubspot: valid api token

</td>
<td>

validation

</td>
<td>

`hubspot_1` - hubspot API key

</td>
</tr>
<tr>
<td colspan="3">

```
keyscope validate hubspot -p HUBSPOT_1
```

</td></tr>
<tr><td>

**lokalise**<br/>lokalise: valid api token

</td>
<td>

validation

</td>
<td>

`lokalise_1` - lokalise token

</td>
</tr>
<tr>
<td colspan="3">

```
keyscope validate lokalise -p LOKALISE_1
```

</td></tr>
</table>
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

## Tutorial: adding Dropbox

To validate if a dropbox API key works, we first need to learn about the canonical way to authenticate against that API.

First stop, API docs:

- Dropbox has an [API Explorer](https://dropbox.github.io/dropbox-api-v2-explorer) which is super useful

Next stop, we want to find an API call that is a representative for:

- Has to be authenticated
- Has to indicate that when accessed successfully with our candidate key, the key has some authoritative value. Which means, that if exposed, contains significant risk.

For this example, getting our current account sounds like something that only when we identify who we are - we're able to do.

We'll select [get_current_account](https://www.dropbox.com/developers/documentation/http/documentation#users-get_current_account).

Let's start forming our interaction. First the needed skeleton: containing the name of the provider (`dropbox`), its ID and description below, as well as parameters required and their name and description:

```yaml
dropbox:
  validation:
    request:
      id: "dropbox:validation"
      desc: "dropbox: valid API credentials"
      params:
        - name: dropbox_1
          desc: dropbox token
```

We keep the name of the parameter with a special convention that helps when feeding keyscope automatically:

```
PROVIDER_N
Where 'N' starts in 1 e.g.:
dropbox_1
dropbox_2
aws_1
...
```

Then, details about actually making an HTTP call, as required by Dropbox (Bearer token authentication).

```yaml
uri: https://api.dropboxapi.com/2/users/get_current_account
method: post
headers:
  Authorization:
    - Bearer {{dropbox_1}}
```

Note that per standard, all HTTP header fields are actually _arrays_. It's OK to always make an array of size _one_ if you only have one value (most common case).

We also see _variable interpolation_ here. Where `{{dropbox_1}}` will get replaced by keyscope in time before making the actual call.

Finally, we want to make sure we answer the question:

- What does it mean to have a successful call?

In our case, the Dropbox API call returns `HTTP OK` on success, which means a `200` status code.

And the final, complete result is this:

```yaml
dropbox:
  validation:
    request:
      id: "dropbox:validation"
      desc: "dropbox: valid API credentials"
      params:
        - name: dropbox_1
          desc: dropbox token
      uri: https://api.dropboxapi.com/2/users/get_current_account
      method: post
      headers:
        Authorization:
          - Bearer {{dropbox_1}}
    response:
      status_code: "200"
```

Meanwhile, you can drop this provider in your own `providers.yaml` file and run keyscope:

```
$ keyscope -f providers.yaml validate dropbox -p MY_KEY
```

Now you can keep this in your private `providers.yaml` file or contribute it back to keyscope if you think other people might enjoy using it - we're happy to accept pull requests.

# Thanks

To all [Contributors](https://github.com/spectralops/keyscope/graphs/contributors) - you make this happen, thanks!

# Copyright

Copyright (c) 2021 [@jondot](http://twitter.com/jondot). See [LICENSE](LICENSE.txt) for further details.
