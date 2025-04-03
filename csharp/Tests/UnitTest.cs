using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    public void TestMethod1()
    {
        int[][] edges = [[4, 1, -1], [2, 0, -1], [0, 3, -1], [4, 3, -1]];
        Assert.AreEqual([[4, 1, 1], [2, 0, 1], [0, 3, 3], [4, 3, 1]], sol.ModifiedGraphEdges(5, edges, 0, 1, 5));
    }

    [TestMethod]
    public void TestMethod2()
    {
        int[][] edges = [[0, 1, -1], [0, 2, 5]];
        Assert.AreEqual([], sol.ModifiedGraphEdges(3, edges, 0, 2, 6));
    }

    [TestMethod]
    public void TestMethod3()
    {
        int[][] edges = [[1, 0, 4], [1, 2, 3], [2, 3, 5], [0, 3, -1]];
        Assert.AreEqual([[1, 0, 4], [1, 2, 3], [2, 3, 5], [0, 3, 1]], sol.ModifiedGraphEdges(4, edges, 0, 2, 6));
    }

    [TestMethod]
    public void TestMethod4()
    {
    }
}