using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new int[] { 1, 2, 3, 3, 4, 4, 5, 6 }, 4, true)]
    [DataRow(new int[] { 3, 2, 1, 2, 3, 4, 3, 4, 5, 9, 10, 11 }, 3, true)]
    [DataRow(new int[] { 1, 2, 3, 4 }, 3, false)]
    public void TestMethod1(int[] nums, int k, bool exp)
    {
        Assert.AreEqual(exp, sol.IsPossibleDivide(nums, k));
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