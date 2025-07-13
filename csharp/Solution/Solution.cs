using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MatchPlayersAndTrainers(int[] players, int[] trainers)
    {
        Array.Sort(players);
        Array.Sort(trainers);
        int i = 0;
        int res = 0;
        foreach (var p in players)
        {
            while (i < trainers.Length && trainers[i] < p) { i += 1; }
            if (i < trainers.Length)
            {
                res += 1;
                i += 1;
            }
            else { break; }
        }
        return res;
    }
}