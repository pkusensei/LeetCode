using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int PeopleAwareOfSecret(int n, int delay, int forget)
    {
        const int M = 1_000_000_007;
        Queue<(int day, int count)> knowq = [];
        Queue<(int day, int count)> shareq = [];
        knowq.Enqueue((1, 1));
        int knows = 1;
        int shares = 0;
        for (int i = 1; i <= n; i++)
        {
            while (knowq.TryPeek(out var item) && i - item.day >= delay)
            {
                (int day, int count) = knowq.Dequeue();
                shareq.Enqueue((day, count));
                knows = (knows - count + M) % M;
                shares = (shares + count) % M;
            }
            while (shareq.TryPeek(out var item) && i - item.day >= forget)
            {
                (int _day, int count) = shareq.Dequeue();
                shares = (shares - count + M) % M;
            }
            if (shares > 0)
            {
                knows = (knows + shares) % M;
                knowq.Enqueue((i, shares));
            }
        }
        return (knows + shares) % M;
    }
}
