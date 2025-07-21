using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<string> FindWords(char[][] board, string[] words)
    {
        Trie trie = new();
        foreach (var s in words)
        {
            trie.Add(s);
        }
        HashSet<string> res = [];
        int rows = board.Length;
        int cols = board[0].Length;
        for (int r = 0; r < rows; r++)
        {
            for (int c = 0; c < cols; c++)
            {
                trie.Backtrack(board, r, c, new(), res);
            }
        }
        return [.. res];
    }
}

public class Trie
{
    public Trie()
    {
        Nodes = new Trie[26];
        IsEnd = false;
    }

    public Trie[] Nodes { get; }
    public bool IsEnd { get; private set; }

    public void Add(ReadOnlySpan<char> s)
    {
        var curr = this;
        foreach (var ch in s)
        {
            curr = curr.Nodes[ch - 'a'] ??= new();
        }
        curr.IsEnd = true;
    }

    public void Backtrack(char[][] board, int row, int col, StringBuilder sb, HashSet<string> res)
    {
        if (IsEnd) { res.Add(sb.ToString()); }
        int rows = board.Length;
        int cols = board[0].Length;
        if (0 <= row && row < rows && 0 <= col && col < cols && char.IsAsciiLetterLower(board[row][col]))
        {
            char temp = board[row][col];
            sb.Append(board[row][col]);
            board[row][col] = '.';
            int idx = temp - 'a';
            foreach (var (dr, dc) in new[] { (-1, 0), (1, 0), (0, -1), (0, 1) })
            {
                int nr = row + dr;
                int nc = col + dc;
                Nodes[idx]?.Backtrack(board, nr, nc, sb, res);
            }
            sb.Remove(sb.Length - 1, 1);
            board[row][col] = temp;
        }
    }
}
