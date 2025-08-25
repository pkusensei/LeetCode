using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(2, "acb", 4, "ab", 2)]
    public void TestMethod1(int exp, string s1, int n1, string s2, int n2)
    {
        Assert.AreEqual(exp, sol.GetMaxRepetitions(s1, n1, s2, n2));
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