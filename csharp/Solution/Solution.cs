using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int[] DeckRevealedIncreasing(int[] deck)
    {
        int n = deck.Length;
        Array.Sort(deck);
        List<int> res = new(n);
        for (int i = n - 1; i >= 0; i -= 1)
        {
            if (res.Count > 0)
            {
                int temp = res[^1];
                res.RemoveAt(res.Count - 1);
                res.Insert(0, temp);
            }
            res.Insert(0, deck[i]);
        }
        return [.. res];
    }
}
