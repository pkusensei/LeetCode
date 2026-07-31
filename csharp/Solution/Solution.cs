using System.Collections.Frozen;
using System.Linq.Expressions;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<int> FindNumOfValidWords(string[] words, string[] puzzles)
    {
        int[] arr = [.. words.Select(s => Mask(s))];
        List<int> res = new(puzzles.Length);
        foreach (var q in puzzles)
        {
            int curr = 0;
            int qmask = Mask(q);
            int front = 1 << (q[0] - 'a');
            for (int i = 0; i < words.Length; i++)
            {
                int mask = arr[i];
                if ((mask & front) == front && (qmask & mask) == mask) { curr += 1; }
            }
            res.Add(curr);
        }
        return res;

        static int Mask(ReadOnlySpan<char> s)
        {
            int res = 0;
            foreach (var c in s)
            {
                res |= 1 << (c - 'a');
            }
            return res;
        }
    }
}

