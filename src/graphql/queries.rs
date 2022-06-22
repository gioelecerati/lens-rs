pub struct Queries {
    pub profile: Profile,
    pub follow: Follow,
    pub user: User,
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

pub struct Module {
    pub follow_module: String,
    pub fee_follow_module: String,
}

pub struct User {
    pub challenge: String,
    pub login: String,
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
        }
    }
}

/// The GraphQL queries are hardcoded as templates here.
/// Could be probably smarter using a proper library for this
/// but this is a quick and dirty solution.
impl Queries {
    pub fn new() -> Queries {
        Queries {
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
            },
        }
    }
}
