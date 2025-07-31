using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Twitter
{
    public Twitter()
    {
        Tweets = [];
        Follows = [];
    }

    List<Tweet> Tweets { get; }
    Dictionary<int, HashSet<int>> Follows { get; }

    public void PostTweet(int userId, int tweetId)
    {
        Tweets.Add(new(userId, tweetId));
        Follows.TryAdd(userId, [userId]);
    }

    public IList<int> GetNewsFeed(int userId)
    {
        if (Follows.TryGetValue(userId, out var fols))
        {
            return [.. Tweets.AsEnumerable().Reverse()
                             .Where(t => fols.Contains(t.User))
                             .Select(t => t.Id).Take(10)];
        }
        else
        {
            Follows.Add(userId, [userId]);
            return [];
        }
    }

    public void Follow(int followerId, int followeeId)
    {
        Follows.TryAdd(followerId, [followerId]);
        Follows[followerId].Add(followeeId);
    }

    public void Unfollow(int followerId, int followeeId)
    {
        Follows.TryAdd(followerId, [followerId]);
        Follows[followerId].Remove(followeeId);
    }
}

internal record struct Tweet(int User, int Id);