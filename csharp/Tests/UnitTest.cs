using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 3, 2, 1 }, new[] { 0, 3, 3 }, 1, 1, 3)]
    [DataRow(new[] { 5, 4 }, new[] { 0, 0, 0 }, 1, 5, 1)]
    [DataRow(new[] { 10, 15, 30 }, new[] { 0, 10, 10, 10, 10 }, 3, 10, 2)]
    public void TestMethod1(int[] tasks, int[] workers, int pills, int strength, int exp)
    {
        Assert.AreEqual(exp, sol.MaxTaskAssign(tasks, workers, pills, strength));
    }

    [TestMethod]
    public void TestMethod2()
    { }

    [TestMethod]
    public void TestMethod3()
    {
    }

    [TestMethod]
    public void TestMethod4()
    {
    }
}