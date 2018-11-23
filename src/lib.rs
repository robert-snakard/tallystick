//! TallyMan is a work-in-progress rust library for tallying votes.
//!
//! ## Compatibility
//!
//! The `tallyman` crate currently needs nightly rust. It will move to stable when NLL is stabilized.

#![warn(rust_2018_idioms)]
#![warn(missing_docs)]
#![feature(crate_visibility_modifier)]
#![feature(nll)]

#[allow(unused_imports)]
#[macro_use] extern crate derive_more;

extern crate hashbrown;
extern crate petgraph;
extern crate num_traits;

#[cfg(feature = "rational")]
extern crate num_rational;


/// Plurality voting is an electoral system in which each voter is allowed to vote for only one candidate 
/// and the candidate who polls the most among their counterparts (a plurality) is elected. It may be called
/// first-past-the-post (FPTP), single-choice voting, simple plurality, or relative/simple majority. 
/// 
/// # Example
/// ```
///    use tallyman::plurality::DefaultTally;
///
///    // Election between Alice, Bob, and Cir with two winners.
///    let mut tally = DefaultTally::new(2);
///    tally.add("Alice");
///    tally.add("Cir");
///    tally.add("Bob");
///    tally.add("Alice");
///    tally.add("Alice");
///    tally.add("Bob");
/// 
///    let winners = tally.winners().into_unranked();
///    println!("The winners are {:?}", winners);
/// ```
pub mod plurality;

/// Approval voting is a single-winner electoral system where each voter may select ("approve") any number of
/// candidates. The winner is the most-approved candidate.
pub mod approval;

/// Score voting or "range voting" is an electoral system in which voters give each candidate a score,
/// the scores are summed, and the candidate with the highest total is elected. It has been described
/// by various other names including "evaluative voting", "utilitarian voting", and "the point system".
pub mod score;


/// The single transferable vote (STV) is a ranked choice voting system.
/// Under STV, a voter has a single vote that is initially allocated to their most preferred candidate. Votes are totalled and a quota
/// (the number of votes required to win) derived. If a candidate achieves quota, the candidate is elected and any surplus vote
/// is transferred to other candidates in proportion to the voters' stated preferences. If no candidate achieves quota, 
/// the bottom candidate is eliminated with votes being transferred to other candidates as determined by the voters' stated preferences.
/// These elections, eliminations, and vote transfers continue in rounds until the correct number of candidates are elected.
pub mod stv;


/// The Condorcet method is a ranked-choice voting system that elects the candidate that would win a majority
/// of the vote in all of the head-to-head elections against each of the other candidates.
/// 
/// The Condorcet method isn't guarunteed to produce a single-winner due to the non-transitive nature of group choice.
pub mod condorcet;


// Common Data Structures
// ----------------------
mod result;
pub use result::RankedWinners;

/// A quota defines how many votes are required to win an election in relation to the total number of votes cast.
pub enum Quota {

    /// Droop quota. It is defined as:
    /// 
    /// ```floor((total-votes / (total-seats + 1)) + 1```
    /// 
    /// In single-winner elections, it's often known as "fifty percent plus one". The Droop quota is always an integer.
    /// 
    /// See [wikipedia](https://en.wikipedia.org/wiki/Droop_quota) for more details.
    Droop,

    /// Hagenbach-Bischoff quota.
    /// 
    /// Also known as the "Newland-Britton quota" or the "exact Droop quota", it is defined as:
    /// 
    /// ```total-votes / (total-seats + 1)```
    /// 
    /// It differs from the Droop quota in that the quota often contains a fraction. In single-winner elections, 
    /// the first candidate to achieve more than 50% of the vote wins. This system is best used when fractional 
    /// votes are being used, or in a transferable-vote system where votes are redistributed fractionally.
    /// 
    /// See [wikipedia](https://en.wikipedia.org/wiki/Hagenbach-Bischoff_quota) for more details.
    Hagenbach,
  
    /// Hare quota.
    /// 
    /// It is defined as:
    /// 
    /// ```total-votes / total-seats```
    /// 
    /// In single-winner elections, it is equal to fifty percent of the vote. It is generally not recommended and
    /// is included for completeness.
    /// 
    /// See [wikipedia](https://en.wikipedia.org/wiki/Hare_quota) for more details.
    Hare
}