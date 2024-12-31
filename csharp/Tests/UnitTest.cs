using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new int[] { 1, 4, 6, 7, 8, 20 }, new int[] { 2, 7, 15 }, 11)]
    [DataRow(new int[] { 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 30, 31 }, new int[] { 2, 7, 15 }, 17)]
    [DataRow(new int[] { 1, 4, 6, 7, 8, 20 }, new int[] { 7, 2, 15 }, 6)]
    public void TestMethod1(int[] days, int[] costs, int exp)
    {
        Assert.AreEqual(exp, sol.MincostTickets(days, costs));
        Assert.AreEqual(exp, sol.TopDown(days, costs));
    }

    [TestMethod]
    public void TestMethod2()
    {
    }

    [TestMethod]
    public void TestMethod3()
    {
    }

    [TestMethod]
    public void TestMethod4()
    {
    }
}