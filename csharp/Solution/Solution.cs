using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int FindMinStep(string board, string hand)
    {
        List<byte> board_ = Compress(board);
        List<byte> hand_ = Compress(hand);
        hand_.Sort();
        Queue<(List<byte> board, List<byte> hand, int step)> queue = [];
        queue.Enqueue((board_, hand_, 0));
        HashSet<(List<byte>, List<byte>)> seen = [(board_, hand_)];
        while (queue.TryDequeue(out var item))
        {
            if (item.board.Count == 0) { return item.step; }
            for (int bi = 0; bi < item.board.Count; bi++)
            {
                for (int hi = 0; hi < item.hand.Count; hi++)
                {
                    if (bi > 0 && item.board[bi - 1] == item.hand[hi]) { continue; }
                    if (hi > 0 && item.hand[hi - 1] == item.hand[hi]) { continue; }
                    if (item.board[bi] == item.hand[hi]
                        || (bi > 0 && item.board[bi - 1] == item.board[bi]))
                    {
                        List<byte> nboard = [.. item.board];
                        List<byte> nhand = [.. item.hand];
                        nboard.Insert(bi, item.hand[hi]);
                        nhand.RemoveAt(hi);
                        nboard = Process(nboard, bi);
                        if (seen.Add((nboard, nhand)))
                        {
                            queue.Enqueue((nboard, nhand, 1 + item.step));
                        }
                    }
                }
            }
        }
        return -1;

        static List<byte> Process(List<byte> s, int i)
        {
            if (i >= s.Count) { return s; }
            int left = i;
            for (; left > 0 && s[left - 1] == s[i]; left -= 1) { }
            int right = i;
            for (; 1 + right < s.Count && s[1 + right] == s[i]; right += 1) { }
            int count = right - left + 1;
            if (count >= 3)
            {
                s.RemoveRange(left, count);
                return Process(s, left);
            }
            return s;
        }

        static List<byte> Compress(ReadOnlySpan<char> s)
        {
            List<byte> res = new(s.Length);
            foreach (var c in s)
            {
                res.Add((byte)(c - 'A'));
            }
            return res;
        }
    }
}
