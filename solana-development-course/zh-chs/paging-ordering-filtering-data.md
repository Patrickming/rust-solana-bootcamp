# Summary

- This lesson delves into some functionality of the RPC calls that we used in the deserializing account data lesson
- To save on compute time, you can fetch a large number of accounts without their data by filtering them to return just an array of public keys
- Once you have a filtered list of public keys, you can order them and fetch the account data they belong to

# Overview

You may have noticed in the last lesson that while we could fetch and display a list of account data, we didn’t have any granular control over how many accounts to fetch or their order. In this lesson, we’ll learn about some configuration options for the `getProgramAccounts` function that will enable things like paging, ordering accounts, and filtering.

## Use `dataSlice` to only fetch data you 

Imagine the Movie Review app we worked on in past lessons having four million movie reviews and that the average review is 500 bytes. That would make the total download for all review accounts over 2GB. Definitely not something you want to have your frontend download every time the page refreshes.

Fortunately, the `getProgramAccounts` function that you use to get all of the accounts takes a configuration object as argument. One of the configuration options is `dataSlice` which lets you provide two things:

- `offset` the offset from the beginning of the data buffer to start the slice
- `length` the number of bytes to return, starting from the provided offset

When you include a `dataSlice` in the configuration object, the function will only return the subset of the data buffer that you specified.

## Paging Accounts

One area this becomes helpful is with paging. If you want to have a list that displays all accounts but there are so many accounts that you don’t want to pull all the data at once, you can fetch all of the accounts but not fetch their data by using a `dataSlice` of `{ offset: 0, length: 0 }`. You can then map the result to a list of account keys whose data you can fetch only when needed.

```tsx
const accountsWithoutData = await connection.getProgramAccounts(
  programId,
  {
    dataSlice: { offset: 0, length: 0 }
  }
)

const accountKeys = accountsWithoutData.map(account => account.pubkey)
```

With this list of keys, you can then fetch account data in “pages” using the `getMultipleAccountsInfo` method:

```tsx
const paginatedKeys = accountKeys.slice(0, 10)
const accountInfos = await connection.getMultipleAccountsInfo(paginatedKeys)
const deserializedObjects = accountInfos.map((accountInfo) => {
  // put logic to deserialize accountInfo.data here
})
```

## Ordering Accounts

The `dataSlice` option is also helpful when you need to order a list of accounts while paging. You still don’t want to fetch all the data at once, but you do need all of the keys and a way to order them up front. In this case, you need to understand the layout of the account data and configure the data slice to only be the data you need to use for ordering.

For example, you might have an account that stores contact information like so:

- `initialized` as a boolean
- `phoneNumber` as an unsigned, 64-bit integer
- `firstName` as a string
- `secondName` as a string

If you want to order all of the account keys alphabetically based on the user’s first name, you need to find out the offset where the name starts. The first field, `initialized`, takes the first byte, then `phoneNumber` takes another 8, so the `firstName` field starts at offset `1 + 8 = 9`. However, dynamic data fields in borsh use the first 4 bytes to record the length of the data, so we can skip an additional 4 bytes, making the offset 13.

You then need to determine the length to make the data slice. Since the length is variable, we can’t know for sure before fetching the data. But you can choose a length that is large enough to cover most cases and short enough to not be too much of a burden to fetch. 15 bytes is plenty for most first names, but would result in a small enough download even with a million users.

Once you’ve fetched accounts with the given data slice, you can use the sort method to `sort` the array before mapping it to an array of public keys.

```tsx
const accounts = await connection.getProgramAccounts(
  programId,
  {
    dataSlice: { offset: 13, length: 15 }
  }
)

  accounts.sort( (a, b) => {
    const lengthA = a.account.data.readUInt32LE(0)
    const lengthB = b.account.data.readUInt32LE(0)
    const dataA = a.account.data.slice(4, 4 + lengthA)
    const dataB = b.account.data.slice(4, 4 + lengthB)
    return dataA.compare(dataB)
  })

const accountKeys = accounts.map(account => account.pubkey)
```

Note that in the snippet above we don’t compare the data as given. This is because for dynamically sized types like strings, Borsh places an unsigned, 32-bit (4 byte) integer at the start to indicate the length of the data representing that field. So to compare the first names directly, we need to get the length for each, then create a data slice with a 4 byte offset and the proper length.

## Use filters to only retrieve specific accounts

Limiting the data received per account is great, but what if you only want to return accounts that match a specific criteria rather than all of them? That’s where the `filters` configuration option comes in. This option is an array that can have objects matching the following:

- `memcmp` compares a provided series of bytes with program account data at a particular offset. Fields:
    - `offset` the number to offset into program account data before comparing data
    - `bytes` a base-58 encoded string representing the data to match; limited to less than 129 bytes
- `dataSize` compares the program account data length with the provided data size

These let you filter based on matching data and/or total data size.

For example, you could search through a list of contacts by including a `memcmp`` filter:

```tsx
async function fetchMatchingContactAccounts(connection: web3.Connection, search: string): Promise<(web3.AccountInfo<Buffer> | null)[]> {
  const accounts = await connection.getProgramAccounts(
    programId,
    {
      dataSlice: { offset: 0, length: 0 },
      filters: [
        {
          memcmp:
            {
              offset: 13,
              bytes: bs58.encode(Buffer.from(search))
            }
        }
      ]
    }
  )
}
```

Two things to note in the example above:

1. We’re setting the offset to 13 because we determined previously that the offset for `firstName` in the data layout is 9 and we want to additionally skip the first 4 bytes indicating the length of the string.
2. We’re using a third party library `bs58` to perform base-58 encoding on the search term. You can install it using `npm install bs58`.

# Lab

Remember that Movie Review app we worked on in the last two lessons? We’re going to spice it up a little by paging the review list, ordering the reviews so they aren’t so random, and adding some basic search functionality. No worries if you’re just jumping into this lesson without having looked at the previous ones - as long as you have the prerequisite knowledge, you should be able to follow the lab without having worked in this specific project yet.

![movie-review-fronted](../../assets/movie-reviews-frontend.png)

## 1. Download the starter code

If you didn’t complete the lab from the last lesson or just want to make sure that you didn’t miss anything, you can download the [starter code](https://github.com/Unboxed-Software/solana-movie-frontend/tree/solution-deserialize-account-data).

The project is a fairly simple Next.js application. It includes the `WalletContextProvider` we created in the Wallets lesson, a `Card` component for displaying a movie review, a `MovieList` component that displays reviews in a list, a `Form` component for submitting a new review, and a `Movie.ts` file that contains a class definition for a `Movie` object.

## Add paging to the reviews
First things first, let’s create a space to encapsulate the code for fetching account data. Create a new file `MovieCoordinator.ts` and declare a `MovieCoordinator` class. Then let’s move the `MOVIE_REVIEW_PROGRAM_ID` constant from `MovieList` into this new file since we’ll be moving all references to it

```tsx
const MOVIE_REVIEW_PROGRAM_ID = 'CenYq6bDRB7p73EjsPEpiYN7uveyPUTdXkDkgUduboaN'

export class MovieCoordinator { }
```

Now we can use `MovieCoordinator` to create a paging implementation. A quick note before we dive in: this will be as simple a paging implementation as possible so that we can focus on the complex part of interacting with Solana accounts. You can, and should, do better for a production application.

With that out of the way, let’s create a static property `accounts` of type `web3.PublicKey[]`, a static function `prefetchAccounts(connection: web3.Connection)`, and a static function `fetchPage (connection: web3.Connection, page: number, perPage: number): Promise<Movie[]>`. You’ll also need to import `@solana/web3.js` and `Movie`.

```tsx
import * as web3 from '@solana/web3.js'
import { Movie } from '../models/Movie'

const MOVIE_REVIEW_PROGRAM_ID = 'CenYq6bDRB7p73EjsPEpiYN7uveyPUTdXkDkgUduboaN'

export class MovieCoordinator {
  static accounts: web3.PublicKey[] = []

  static async prefetchAccounts(connection: web3.Connection) {

  }

  static async fetchPage(connection: web3.Connection, page: number, perPage: number): Promise<Movie[]> {

  }
}
```

The key to paging is to prefetch all the accounts without data. Let’s fill in the body of `prefetchAccounts` to do this and set the retrieved public keys to the static `accounts` property.

```tsx
static async prefetchAccounts(connection: web3.Connection) {
  const accounts = await connection.getProgramAccounts(
    new web3.PublicKey(MOVIE_REVIEW_PROGRAM_ID),
    {
      dataSlice: { offset: 0, length: 0 },
    }
  )

  this.accounts = accounts.map(account => account.pubkey)
}
```

