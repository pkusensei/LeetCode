using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public Solution(double radius, double x_center, double y_center)
    {
        Radius = radius;
        X = x_center;
        Y = y_center;
        Rng = new Random();
    }

    public double Radius { get; }
    public double X { get; }
    public double Y { get; }
    public Random Rng { get; }

    public double[] RandPoint()
    {
        double angle = Rng.NextDouble() * double.Pi * 2;
        double r = Radius * double.Sqrt(Rng.NextDouble());
        return [X + r * double.Cos(angle), Y + r * double.Sin(angle)];
    }
}