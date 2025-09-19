using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Spreadsheet
{
    public Spreadsheet(int rows)
    {
        Table = new int[rows, 26];
    }

    int[,] Table { get; }

    public void SetCell(string cell, int value)
    {
        int col = cell[0] - 'A';
        int row = int.Parse(cell[1..]) - 1;
        Table[row, col] = value;
    }

    public void ResetCell(string cell) => SetCell(cell, 0);

    public int GetValue(string formula)
    {
        int res = 0;
        foreach (var item in formula[1..].Split('+'))
        {
            if (int.TryParse(item, out var v)) { res += v; }
            else
            {
                int col = item[0] - 'A';
                int row = int.Parse(item[1..]) - 1;
                res += Table[row, col];
            }
        }
        return res;
    }
}
