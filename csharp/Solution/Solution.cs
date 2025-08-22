using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int PoorPigs(int buckets, int minutesToDie, int minutesToTest)
    {
        // Each pig can try this many buckets
        int attempt = minutesToTest / minutesToDie;
        // Plus one that's left out if they are still alive
        // This is the number each pig can select
        // Or pieces of information each can gather
        int choices = 1 + attempt;
        int res = 0;
        for (; Math.Pow(choices, res) < buckets; res += 1) { }
        return res;
    }
}