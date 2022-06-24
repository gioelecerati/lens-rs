use serde::{Deserialize, Serialize};

pub struct Queries {
    pub ping: super::Query,
    pub profile: Profile,
    pub follow: Follow,
    pub user: User,
    pub publication: Publication,
}

pub struct Profile {
    pub create_profile: String,
    pub get_default_profile: String,
    pub get_profiles: String,
    pub recommended_profiles: String,
}

pub struct Follow {
    pub does_follow: String,
    pub get_following: String,
    pub get_followers: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Module {
    pub follow_module: String,
    pub fee_follow_module: String,
    pub only_comments: String,
    pub only_posts: String,
    pub only_mirrors: String,
    pub mirrors_and_comments: String,
    pub post_and_comments: String,
    pub all_publications: String,
}

pub struct User {
    pub challenge: String,
    pub login: String,
    pub verify: String,
    pub refresh: String,
}

pub struct Publication {
    pub get_publications: String,
}

impl Module {
    pub fn new() -> Module {
        Module {
            follow_module: String::from(
                r#"{
                freeFollowModule: true
             }"#,
            ),
            fee_follow_module: String::from(
                r#"{
                feeFollowModule: {
                       amount: {
                           currency: "%%CURRENCY%%",
                           value: "%%VALUE%%"
                       },
                       recipient: "%%RECIPIENT%%"
                }
            }"#,
            ),
            only_comments: String::from(r#"[COMMENT]"#),
            only_mirrors: String::from(r#"[MIRROR]"#),
            only_posts: String::from(r#"[POST]"#),
            post_and_comments: String::from(r#"[POST, COMMENT]"#),
            mirrors_and_comments: String::from(r#"[MIRROR, COMMENT]"#),
            all_publications: String::from(r#"[POST, MIRROR, COMMENT]"#),
        }
    }
}

/// The GraphQL queries are hardcoded as templates here.
/// Could be probably smarter using a proper library for this
/// but this is a quick and dirty solution.
impl Queries {
    pub fn new() -> Queries {
        Queries {
            ping: super::Query {
                query: String::from(
                    r#"query Query {
                  ping
                }"#,
                ),
            },
            publication: Publication {
                get_publications: String::from(
                    r#"query Publications {
                  publications(request: {
                    profileId: "%%PROFILE_ID%%",
                    publicationTypes: %%PUBLICATION_TYPES%%,
                    limit: %%LIMIT%%
                  }) {
                    items {
                      __typename 
                      ... on Post {
                        ...PostFields
                      }
                      ... on Comment {
                        ...CommentFields
                      }
                      ... on Mirror {
                        ...MirrorFields
                      }
                    }
                    pageInfo {
                      prev
                      next
                      totalCount
                    }
                  }
                }
                
                fragment MediaFields on Media {
                  url
                  mimeType
                }
                
                fragment ProfileFields on Profile {
                  id
                  name
                  bio
                  attributes {
                     displayType
                     traitType
                     key
                     value
                  }
                  isFollowedByMe
                  isFollowing(who: null)
                  followNftAddress
                  metadata
                  isDefault
                  handle
                  picture {
                    ... on NftImage {
                      contractAddress
                      tokenId
                      uri
                      verified
                    }
                    ... on MediaSet {
                      original {
                        ...MediaFields
                      }
                    }
                  }
                  coverPicture {
                    ... on NftImage {
                      contractAddress
                      tokenId
                      uri
                      verified
                    }
                    ... on MediaSet {
                      original {
                        ...MediaFields
                      }
                    }
                  }
                  ownedBy
                  dispatcher {
                    address
                  }
                  stats {
                    totalFollowers
                    totalFollowing
                    totalPosts
                    totalComments
                    totalMirrors
                    totalPublications
                    totalCollects
                  }
                  followModule {
                    ... on FeeFollowModuleSettings {
                      type
                      amount {
                        asset {
                          name
                          symbol
                          decimals
                          address
                        }
                        value
                      }
                      recipient
                    }
                    ... on ProfileFollowModuleSettings {
                     type
                    }
                    ... on RevertFollowModuleSettings {
                     type
                    }
                  }
                }
                
                fragment PublicationStatsFields on PublicationStats { 
                  totalAmountOfMirrors
                  totalAmountOfCollects
                  totalAmountOfComments
                }
                
                fragment MetadataOutputFields on MetadataOutput {
                  name
                  description
                  content
                  media {
                    original {
                      ...MediaFields
                    }
                  }
                  attributes {
                    displayType
                    traitType
                    value
                  }
                }
                
                fragment Erc20Fields on Erc20 {
                  name
                  symbol
                  decimals
                  address
                }
                
                fragment CollectModuleFields on CollectModule {
                  __typename
                  ... on FreeCollectModuleSettings {
                      type
                      followerOnly
                      contractAddress
                  }
                  ... on FeeCollectModuleSettings {
                    type
                    amount {
                      asset {
                        ...Erc20Fields
                      }
                      value
                    }
                    recipient
                    referralFee
                  }
                  ... on LimitedFeeCollectModuleSettings {
                    type
                    collectLimit
                    amount {
                      asset {
                        ...Erc20Fields
                      }
                      value
                    }
                    recipient
                    referralFee
                  }
                  ... on LimitedTimedFeeCollectModuleSettings {
                    type
                    collectLimit
                    amount {
                      asset {
                        ...Erc20Fields
                      }
                      value
                    }
                    recipient
                    referralFee
                    endTimestamp
                  }
                  ... on RevertCollectModuleSettings {
                    type
                  }
                  ... on TimedFeeCollectModuleSettings {
                    type
                    amount {
                      asset {
                        ...Erc20Fields
                      }
                      value
                    }
                    recipient
                    referralFee
                    endTimestamp
                  }
                }
                
                fragment PostFields on Post {
                  id
                  profile {
                    ...ProfileFields
                  }
                  stats {
                    ...PublicationStatsFields
                  }
                  metadata {
                    ...MetadataOutputFields
                  }
                  createdAt
                  collectModule {
                    ...CollectModuleFields
                  }
                  referenceModule {
                    ... on FollowOnlyReferenceModuleSettings {
                      type
                    }
                  }
                  appId
                  hidden
                  reaction(request: null)
                  mirrors(by: null)
                  hasCollectedByMe
                }
                
                fragment MirrorBaseFields on Mirror {
                  id
                  profile {
                    ...ProfileFields
                  }
                  stats {
                    ...PublicationStatsFields
                  }
                  metadata {
                    ...MetadataOutputFields
                  }
                  createdAt
                  collectModule {
                    ...CollectModuleFields
                  }
                  referenceModule {
                    ... on FollowOnlyReferenceModuleSettings {
                      type
                    }
                  }
                  appId
                  hidden
                  reaction(request: null)
                  hasCollectedByMe
                }
                
                fragment MirrorFields on Mirror {
                  ...MirrorBaseFields
                  mirrorOf {
                   ... on Post {
                      ...PostFields          
                   }
                   ... on Comment {
                      ...CommentFields          
                   }
                  }
                }
                
                fragment CommentBaseFields on Comment {
                  id
                  profile {
                    ...ProfileFields
                  }
                  stats {
                    ...PublicationStatsFields
                  }
                  metadata {
                    ...MetadataOutputFields
                  }
                  createdAt
                  collectModule {
                    ...CollectModuleFields
                  }
                  referenceModule {
                    ... on FollowOnlyReferenceModuleSettings {
                      type
                    }
                  }
                  appId
                  hidden
                  reaction(request: null)
                  mirrors(by: null)
                  hasCollectedByMe
                }
                
                fragment CommentFields on Comment {
                  ...CommentBaseFields
                  mainPost {
                    ... on Post {
                      ...PostFields
                    }
                    ... on Mirror {
                      ...MirrorBaseFields
                      mirrorOf {
                        ... on Post {
                           ...PostFields          
                        }
                        ... on Comment {
                           ...CommentMirrorOfFields        
                        }
                      }
                    }
                  }
                }
                
                fragment CommentMirrorOfFields on Comment {
                  ...CommentBaseFields
                  mainPost {
                    ... on Post {
                      ...PostFields
                    }
                    ... on Mirror {
                       ...MirrorBaseFields
                    }
                  }
                }"#,
                ),
            },
            profile: Profile {
                create_profile: String::from(
                    r#"mutation CreateProfile {
                    createProfile(request:{ 
                                  handle: "%%HANDLE%%",
                                  profilePictureUri: %%PROFILE_PICTURE_URI%%,
                                  followNFTURI: null,
                                  followModule: %%FOLLOW_MODULE%%
                                  }) {
                      ... on RelayerResult {
                        txHash
                      }
                      ... on RelayError {
                        reason
                      }
                      __typename
                    }
                  }"#,
                ),
                get_default_profile: String::from(
                    r#"query DefaultProfile {
                    defaultProfile(request: { ethereumAddress: "%%ADDRESS%%"}) {
                      id
                      name
                      bio
                      isDefault
                      attributes {
                        displayType
                        traitType
                        key
                        value
                      }
                      metadata
                      handle
                      picture {
                        ... on NftImage {
                          contractAddress
                          tokenId
                          uri
                          chainId
                          verified
                        }
                        ... on MediaSet {
                          original {
                            url
                            mimeType
                          }
                        }
                      }
                      coverPicture {
                        ... on NftImage {
                          contractAddress
                          tokenId
                          uri
                          chainId
                          verified
                        }
                        ... on MediaSet {
                          original {
                            url
                            mimeType
                          }
                        }
                      }
                      ownedBy
                      dispatcher {
                        address
                        canUseRelay
                      }
                      stats {
                        totalFollowers
                        totalFollowing
                        totalPosts
                        totalComments
                        totalMirrors
                        totalPublications
                        totalCollects
                      }
                      followModule {
                        ... on FeeFollowModuleSettings {
                          type
                          contractAddress
                          amount {
                            asset {
                              name
                              symbol
                              decimals
                              address
                            }
                            value
                          }
                          recipient
                        }
                        ... on ProfileFollowModuleSettings {
                         type
                        }
                        ... on RevertFollowModuleSettings {
                         type
                        }
                      }
                    }
                  }"#,
                ),
                get_profiles: String::from(
                    r#"query Profiles {
                    profiles(request: { ownedBy: ["%%ADDRESS%%"], limit: 10 }) {
                      items {
                        id
                        name
                        bio
                        attributes {
                          displayType
                          traitType
                          key
                          value
                        }
                        metadata
                        isDefault
                        picture {
                          ... on NftImage {
                            contractAddress
                            tokenId
                            uri
                            verified
                          }
                          ... on MediaSet {
                            original {
                              url
                              mimeType
                            }
                          }
                          __typename
                        }
                        handle
                        coverPicture {
                          ... on NftImage {
                            contractAddress
                            tokenId
                            uri
                            verified
                          }
                          ... on MediaSet {
                            original {
                              url
                              mimeType
                            }
                          }
                          __typename
                        }
                        ownedBy
                        dispatcher {
                          address
                          canUseRelay
                        }
                        stats {
                          totalFollowers
                          totalFollowing
                          totalPosts
                          totalComments
                          totalMirrors
                          totalPublications
                          totalCollects
                        }
                        followModule {
                          ... on FeeFollowModuleSettings {
                            type
                            amount {
                              asset {
                                symbol
                                name
                                decimals
                                address
                              }
                              value
                            }
                            recipient
                          }
                          ... on ProfileFollowModuleSettings {
                           type
                          }
                          ... on RevertFollowModuleSettings {
                           type
                          }
                        }
                      }
                      pageInfo {
                        prev
                        next
                        totalCount
                      }
                    }
                  }"#,
                ),
                recommended_profiles: String::from(
                    r#"
                query RecommendedProfiles {
                  recommendedProfiles {
                        id
                      name
                      bio
                      attributes {
                        displayType
                        traitType
                        key
                        value
                      }
                        followNftAddress
                      metadata
                      isDefault
                      picture {
                        ... on NftImage {
                          contractAddress
                          tokenId
                          uri
                          verified
                        }
                        ... on MediaSet {
                          original {
                            url
                            mimeType
                          }
                        }
                        __typename
                      }
                      handle
                      coverPicture {
                        ... on NftImage {
                          contractAddress
                          tokenId
                          uri
                          verified
                        }
                        ... on MediaSet {
                          original {
                            url
                            mimeType
                          }
                        }
                        __typename
                      }
                      ownedBy
                      dispatcher {
                        address
                        canUseRelay
                      }
                      stats {
                        totalFollowers
                        totalFollowing
                        totalPosts
                        totalComments
                        totalMirrors
                        totalPublications
                        totalCollects
                      }
                      followModule {
                        ... on FeeFollowModuleSettings {
                          type
                          amount {
                            asset {
                              symbol
                              name
                              decimals
                              address
                            }
                            value
                          }
                          recipient
                        }
                        ... on ProfileFollowModuleSettings {
                         type
                        }
                        ... on RevertFollowModuleSettings {
                         type
                        }
                      }
                  }
                }"#,
                ),
            },
            follow: Follow {
                does_follow: String::from(
                    r#"query DoesFollow {
                    doesFollow(request: { 
                                  followInfos: [
                                    {
                                      followerAddress: "%%ADDRESS%%",
                                      profileId: "%%PROFILE_ID%%"
                                    }
                                  ] 
                               }) {
                      followerAddress
                      profileId
                      follows
                    }
                  }"#,
                ),
                get_following: String::from(
                    r#"query Following {
                    following(request: { 
                                  address: "%%ADDRESS%%",
                                limit: %%LIMIT%%
                               }) {
                      items {
                        profile {
                          id
                          name
                          bio
                          attributes {
                              displayType
                              traitType
                              key
                              value
                          }
                          metadata
                          isDefault
                          handle
                          picture {
                            ... on NftImage {
                              contractAddress
                              tokenId
                              uri
                              verified
                            }
                            ... on MediaSet {
                              original {
                                url
                                width
                                height
                                mimeType
                              }
                              medium {
                                url
                                width
                                height
                                mimeType
                              }
                              small {
                                url
                                width
                                height
                                mimeType
                              }
                            }
                          }
                          coverPicture {
                            ... on NftImage {
                              contractAddress
                              tokenId
                              uri
                              verified
                            }
                            ... on MediaSet {
                              original {
                                url
                                width
                                height
                                mimeType
                              }
                              small {
                                width
                                url
                                height
                                mimeType
                              }
                              medium {
                                url
                                width
                                height
                                mimeType
                              }
                            }
                          }
                          ownedBy
                          dispatcher {
                            address
                            canUseRelay
                          }
                          stats {
                            totalFollowers
                            totalFollowing
                            totalPosts
                            totalComments
                            totalMirrors
                            totalPublications
                            totalCollects
                          }
                          followModule {
                            ... on FeeFollowModuleSettings {
                              type
                              amount {
                                asset {
                                  name
                                  symbol
                                  decimals
                                  address
                                }
                                value
                              }
                              recipient
                            }
                            ... on ProfileFollowModuleSettings {
                             type
                            }
                            ... on RevertFollowModuleSettings {
                             type
                            }
                          }
                        }
                        totalAmountOfTimesFollowing
                      }
                      pageInfo {
                        prev
                        next
                        totalCount
                      }
                    }
                  }"#,
                ),
                get_followers: String::from(
                    r#"query Followers {
                    followers(request: { 
                                  profileId: "%%PROFILE_ID%%",
                                limit: %%LIMIT%%
                               }) {
                         items {
                        wallet {
                          address
                          defaultProfile {
                            id
                            name
                            bio
                            attributes {
                              displayType
                              traitType
                              key
                              value
                            }
                              metadata
                            isDefault
                            handle
                            picture {
                              ... on NftImage {
                                contractAddress
                                tokenId
                                uri
                                verified
                              }
                              ... on MediaSet {
                                original {
                                  url
                                  mimeType
                                }
                              }
                            }
                            coverPicture {
                              ... on NftImage {
                                contractAddress
                                tokenId
                                uri
                                verified
                              }
                              ... on MediaSet {
                                original {
                                  url
                                  mimeType
                                }
                              }
                            }
                            ownedBy
                            dispatcher {
                              address
                              canUseRelay
                            }
                            stats {
                              totalFollowers
                              totalFollowing
                              totalPosts
                              totalComments
                              totalMirrors
                              totalPublications
                              totalCollects
                            }
                            followModule {
                              ... on FeeFollowModuleSettings {
                                type
                                contractAddress
                                amount {
                                  asset {
                                    name
                                    symbol
                                    decimals
                                    address
                                  }
                                  value
                                }
                                recipient
                              }
                              ... on ProfileFollowModuleSettings {
                               type
                              }
                              ... on RevertFollowModuleSettings {
                               type
                              }
                            }
                          }
                        }
                        totalAmountOfTimesFollowed
                      }
                      pageInfo {
                        prev
                        next
                        totalCount
                      }
                    }
                  }"#,
                ),
            },
            user: User {
                challenge: String::from(
                    r#"query Challenge {
                    challenge(request: { address: "%%ADDRESS%%" }) {
                      text
                    }
                  }"#,
                ),
                login: String::from(
                    r#"mutation Authenticate {
                  authenticate(request: {
                    address: "%%ADDRESS%%",
                    signature: "%%SIGNATURE%%"
                  }) {
                    accessToken
                    refreshToken
                  }
                }"#,
                ),
                verify: String::from(
                    r#"query Query {
                      verify(request: {
                        "accessToken": "%%ACCESS_TOKEN%%"
                      })
                    }"#,
                ),
                refresh: String::from(
                    r#"mutation Refresh {
                      refresh(request: {
                        refreshToken: "%%REFRESH_TOKEN%%"}) {
                        accessToken
                        refreshToken
                      }
                    }"#,
                ),
            },
        }
    }
}
