using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(11, new[] { 1, 4, 6, 7, 8, 20 }, new[] { 2, 7, 15 })]
    public void TestMethod1(int exp, int[] days, int[] costs)
    {
        Assert.AreEqual(exp, sol.MincostTickets(days, costs));
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