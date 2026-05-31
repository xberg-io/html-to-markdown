---
meta-article:published_time: 2017-05-15T02:40:25-06:00
title: Three-Way Analysis of Variance: Simple Second-Order Interaction Effects and Simple Main Effects | R-bloggers
---


[R-bloggersR news and tutorials contributed by hundreds of R bloggers][1]

## Three-Way Analysis of Variance: Simple Second-Order Interaction Effects and Simple Main Effects

Posted on May 15, 2017 by [Bogdan Anastasiei][2] in [R bloggers][3] | 0 Comments

### [This article was first published on **[R-posts.com][4]**, and kindly contributed to [R-bloggers][5]]. (You can report issue about the content on this page [here][6])

---

Want to share your content on R-bloggers?[click here][7] if you have a blog, or [here][8] if you don't.



[Share][9][Tweet][10]

In this article we will show how to run a three-way analysis of variance when both the third-order interaction effect and the second-order interaction effects are statistically significant. This type of analysis can become pretty tedious, especially when our factors have many levels, so we will try to explain it here as clearly as possible. (If you want to watch me doing these analyses live, [get my free course on statistical analysis with R here][11].)

First of all, let’s present the fictitious data we are going to work with. Let’s suppose that a pharmaceutical company is planning to launch a new vitamin that allegedly improves the employees’ resistance to effort. The vitamin is tested on a sample of 720 employees, divided into three groups: employees who take a placebo (the control group), employees who take the vitamin in low dose and employees who take the vitamin in high dose. Half of the employees are male, and half are female. Moreover, we have both blue collar employees and white collar employees in our sample.

The resistance to effort is measured on a scale whatsoever, from 1 to 30 (30 being the highest resistance). Our goal is to determine whether the effort resistance is influenced by three factors: dose of vitamin (placebo, low dose, and high dose), gender (male, female) and type of employee (blue collar, white collar). You can find the experiment data in CSV format [here][12].

**Third-order interaction effect** First of all, let’s check whether the third-order interaction effect is significant. We are going to run the analysis using the *aov* function in the *stats* package (our data frame is called *vitamin*).

```text
aov1 <- aov(effort~dose*gender*type, data=vitamin)
summary(aov1)
```

In the formula above the interaction effect is, of course, *dose*gender*type*. The ANOVA results can be seen below (we have only kept the line presenting the third-order interaction effect).

```text
                                  Df Sum Sq Mean Sq F value   Pr(>F)


dose:gender:type   2    187    93.4  22.367 3.81e-10
```

The interaction effect is statistically significant: F(2)=22.367, p<0.01. In other words, we do have a third-order interaction effect.

In this situation, it is not advisable to report and interpret the second-order interaction effects (they could be misleading). Therefore, we are going to compute the *simple second-order interaction effects*.

**Simple second-order interaction effects** The simple second-order interaction effects are the effects of each pair of factors at each level of the third factor. Specifically, we have to compute the following effects:

1. the interaction effect of dose and type of employee, for each gender category (male and female)
2. the interaction effect of gender and type of employee, at each dose level (placebo, low and high)
3. the interaction effect of dose and gender, for each type of employee (blue collar and white collar).
The total number of second-order interaction effect is given by the sum of the factor levels. In our case we have 7 effects (3+2+2). We will not analyze of all them, because this article would become too long. We will only focus on the first set of effects, leaving the others for you as an exercise.

So let’s investigate the interaction effect of dose and type of employee for each gender group. First we have to create two separate data frames, for male and female employees. We do that with the *filter* command in the *dplyr* package (though you can also use brackets or subsets).

```text
vitamin_male <- filter(vitamin, gender=="male")
vitamin_female <- filter(vitamin, gender=="female")
```

Now we perform a two-way analysis of variance on each data frame (the factors being dose and type, of course).


aov1 <- aov(effort~dose*type, data=vitamin_male)
summary(aov1)

aov2 <- aov(effort~dose*type, data=vitamin_female)
summary(aov2)


The results of the analyses are shown below (we have only retained the lines with the interaction effects).

```text
               Df Sum Sq Mean Sq F value   Pr(>F)   

dose:type     2    249   124.7   28.42 3.57e-12            

                   Df Sum Sq Mean Sq F value   Pr(>F)   

dose:type     2  137.2    68.6   17.31 6.74e-08
```


We can notice that both simple second-order interaction effects are significant (p<0.01). So we are dealing with a combined influence of the factors dose and type of employee in both male and female groups. In this situation, we have to examine the *simple main effects* for each factor. This is what we are going to do in the next section.

**Simple main effects** Let’s compute the main effect for the factor dose of vitamin, which is the most important (after all, the company wants to demonstrate that the vitamin does affect the resistance to effort). You will be able to compute the other simple main effects yourself, using this as an example.

Now we must create four separate data frames, for each combination of the factors gender and type of employee: male – blue collar, male – white collar, female – blue collar, female – white collar.


vitamin_male_blue <- filter(vitamin, gender=="male", type=="blue collar")

vitamin_male_white <- filter(vitamin, gender=="male", type=="white collar")

vitamin_female_blue <- filter(vitamin, gender=="female", type=="blue collar")

vitamin_female_white <- filter(vitamin, gender=="female", type=="white collar")


Next we perform a one-way ANOVA for each data frame. Let’s do it for the first group, male – blue collar.


aov1 <- aov(effort~dose, data=vitamin_male_blue)
summary(aov1)

```text
                Df Sum Sq Mean Sq F value Pr(>F)   

dose          2 2943.5  1471.8   349.9 <2e-16
```


The simple main effect for the factor dose on this group is statistically significant (p<0.01). In other words, there is a significant difference between placebo, low dose and high dose levels within the male – blue collar employees category, regarding the resistance to effort. To find out how big the differences are, we use the *TuckeyHSD* function to compute the test with the same name.

```text
TukeyHSD(aov1)


                                              diff        lwr       upr   p adj

 low dose-high dose -2.528333  -3.413363 -1.643303     0

 placebo-high dose  -9.558333 -10.443363 -8.673303     0

 placebo-low dose   -7.030000  -7.915030 -6.144970     0
```


By inspection of the table we conclude that the differences in effort resistance between the dose groups are significant (p<0.01). The highest difference, in absolute values, is that between low dose and placebo levels: 9.5 points. So the employees who took a high dose present a higher resistance to effort than those who just took a placebo.

One more example: the simple main effects of the variable dose of vitamin on the female – blue collar group.


aov1 <- aov(effort~dose, data=vitamin_female_blue)
summary(aov1)


 Df Sum Sq Mean Sq F value Pr(>F)
dose 2 399.6 199.81 45.57 <2e-16

```text
TukeyHSD(aov1)


                                            diff        lwr       upr     p adj

 low dose-high dose  1.083333  0.1797508  1.986916 0.0141485

 placebo-high dose  -2.476667 -3.3802492 -1.573084 0.0000000

 placebo-low dose   -3.560000 -4.4635826 -2.656417 0.0000000
```

The simple main effect is statistically significant, as it results from the first table. Furthermore, all the differences between dose levels are significant. The highest difference is the difference between low dose and placebo (3.5 points).

To learn more on data analysis in R, [check the free “Statistics with R” video course here.][11]

#### *Related*

[Share][9][Tweet][10]

To **leave a comment** for the author, please follow the link and comment on their blog: **[R-posts.com][4]**.

---

[R-bloggers.com][5] offers **[daily e-mail updates][13]** about [R][14] news and tutorials about [learning R][15] and many other topics. [Click here if you're looking to post or find an R/data-science job][16].

---

Want to share your content on R-bloggers?[click here][7] if you have a blog, or [here][8] if you don't.





[https://feeds.feedburner.com/RBloggers][18]

> [R bloggers Facebook page][19]

— <https://www.facebook.com/rbloggers/>

##### Most viewed posts (weekly)

- [PCA vs Autoencoders for Dimensionality Reduction][20]
- [5 Ways to Subset a Data Frame in R][21]
- [How to write the first for loop in R][22]
- [How to Calculate a Cumulative Average in R][23]
- [Date Formats in R][24]
- [Complete tutorial on using 'apply' functions in R][25]
- [R – Sorting a data frame by the contents of a column][26]

##### Sponsors



##### Recent Posts

- [Something to note when using the merge function in R][27]
- [Better Sentiment Analysis with sentiment.ai][28]
- [Self-documenting plots in ggplot2][29]
- [Data Challenges for R Users][30]
- [simplevis: new & improved!][31]
- [Checking the inputs of your R functions][32]
- [Imputing missing values in R][33]
- [Creating a Dashboard Framework with AWS (Part 1)][34]
- [BensstatsTalks#3: 5 Tips for Landing a Data Professional Role][35]
- [Live COVID-19 Swiss vaccination analysis][36]
- [Complete tutorial on using ‘apply’ functions in R][37]
- [Getting to know Julia][38]
- [Bootstraps & Bandings][39]
- [How to Calculate a Cumulative Average in R][40]
- [Some thoughts about the use of cloud services and web APIs in social science research][41]

##### [https://feeds.feedburner.com/Rjobs][42] [Jobs for R-users][43]

- [Junior Data Scientist / Quantitative economist][44]
- [Senior Quantitative Analyst][45]
- [R programmer][46]
- [Data Scientist – CGIAR Excellence in Agronomy (Ref No: DDG-R4D/DS/1/CG/EA/06/20)][47]
- [Data Analytics Auditor, Future of Audit Lead @ London or Newcastle][48]

##### [https://feeds.feedburner.com/Python-bloggers][49] [python-bloggers.com (python/data-science news)][50]

- [Dunn Index for K-Means Clustering Evaluation][51]
- [Installing Python and Tensorflow with Jupyter Notebook Configurations][52]
- [How to Get Twitter Data using Python][53]
- [Visualizations with Altair][54]
- [Spelling Corrector Program in Python][55]
- [Spelling Checker Program in Python][56]
- [Streamlit Tutorial: How to Deploy Streamlit Apps on RStudio Connect][57]



**[Full list of contributing R-bloggers][58]**

##### Archives

		Archives

Select Month
March 2022 (35)
February 2022 (152)
January 2022 (154)
December 2021 (172)
November 2021 (145)
October 2021 (199)
September 2021 (202)
August 2021 (153)
July 2021 (171)
June 2021 (195)
May 2021 (196)
April 2021 (183)
March 2021 (221)
February 2021 (225)
January 2021 (252)
December 2020 (295)
November 2020 (266)
October 2020 (276)
September 2020 (249)
August 2020 (226)
July 2020 (268)
June 2020 (231)
May 2020 (332)
April 2020 (326)
March 2020 (279)
February 2020 (259)
January 2020 (250)
December 2019 (241)
November 2019 (214)
October 2019 (230)
September 2019 (227)
August 2019 (270)
July 2019 (258)
June 2019 (242)
May 2019 (272)
April 2019 (289)
March 2019 (302)
February 2019 (259)
January 2019 (282)
December 2018 (257)
November 2018 (285)
October 2018 (298)
September 2018 (285)
August 2018 (266)
July 2018 (327)
June 2018 (296)
May 2018 (315)
April 2018 (296)
March 2018 (287)
February 2018 (239)
January 2018 (328)
December 2017 (260)
November 2017 (265)
October 2017 (287)
September 2017 (287)
August 2017 (328)
July 2017 (279)
June 2017 (312)
May 2017 (341)
April 2017 (319)
March 2017 (364)
February 2017 (312)
January 2017 (364)
December 2016 (345)
November 2016 (288)
October 2016 (298)
September 2016 (249)
August 2016 (280)
July 2016 (322)
June 2016 (259)
May 2016 (288)
April 2016 (258)
March 2016 (295)
February 2016 (261)
January 2016 (334)
December 2015 (300)
November 2015 (234)
October 2015 (255)
September 2015 (232)
August 2015 (261)
July 2015 (240)
June 2015 (205)
May 2015 (228)
April 2015 (203)
March 2015 (256)
February 2015 (207)
January 2015 (237)
December 2014 (230)
November 2014 (219)
October 2014 (212)
September 2014 (253)
August 2014 (214)
July 2014 (226)
June 2014 (234)
May 2014 (238)
April 2014 (256)
March 2014 (286)
February 2014 (266)
January 2014 (260)
December 2013 (261)
November 2013 (237)
October 2013 (233)
September 2013 (214)
August 2013 (223)
July 2013 (254)
June 2013 (271)
May 2013 (260)
April 2013 (278)
March 2013 (277)
February 2013 (293)
January 2013 (340)
December 2012 (306)
November 2012 (274)
October 2012 (304)
September 2012 (268)
August 2012 (262)
July 2012 (247)
June 2012 (297)
May 2012 (283)
April 2012 (295)
March 2012 (304)
February 2012 (264)
January 2012 (278)
December 2011 (251)
November 2011 (261)
October 2011 (280)
September 2011 (187)
August 2011 (258)
July 2011 (219)
June 2011 (224)
May 2011 (239)
April 2011 (267)
March 2011 (249)
February 2011 (203)
January 2011 (209)
December 2010 (188)
November 2010 (172)
October 2010 (219)
September 2010 (185)
August 2010 (203)
July 2010 (175)
June 2010 (167)
May 2010 (164)
April 2010 (152)
March 2010 (165)
February 2010 (135)
January 2010 (121)
December 2009 (126)
November 2009 (66)
October 2009 (87)
September 2009 (65)
August 2009 (56)
July 2009 (64)
June 2009 (54)
May 2009 (35)
April 2009 (38)
March 2009 (40)
February 2009 (33)
January 2009 (42)
December 2008 (16)
November 2008 (14)
October 2008 (10)
September 2008 (8)
August 2008 (11)
July 2008 (7)
June 2008 (8)
May 2008 (8)
April 2008 (4)
March 2008 (5)
February 2008 (6)
January 2008 (10)
December 2007 (3)
November 2007 (5)
October 2007 (9)
September 2007 (7)
August 2007 (21)
July 2007 (9)
June 2007 (3)
May 2007 (3)
April 2007 (1)
March 2007 (5)
February 2007 (4)
November 2006 (1)
October 2006 (2)
August 2006 (3)
July 2006 (1)
June 2006 (1)
May 2006 (3)
April 2006 (1)
March 2006 (1)
February 2006 (5)
January 2006 (1)
October 2005 (1)
September 2005 (3)
May 2005 (1)

##### Other sites

- [Jobs for R-users][43]
- [SAS blogs][59]

Copyright © 2022 | [MH Corporate basic by MH Themes][60]

## *Never miss an update!* **Subscribe to R-bloggers** to receive e-mails with the latest R posts. (You will not see this message again.)



[Click here to close (This popup will not appear again)][61]

[1]: https://www.r-bloggers.com/ "R-bloggers"
[2]: https://www.r-bloggers.com/author/bogdan-anastasiei/
[3]: https://www.r-bloggers.com/category/r-bloggers/
[4]: http://r-posts.com/three-way-analysis-of-variance-simple-second-order-interaction-effects-and-simple-main-effects/
[5]: https://www.r-bloggers.com/
[6]: https://www.r-bloggers.com/contact-us/
[7]: https://www.r-bloggers.com/add-your-blog/
[8]: http://r-posts.com/
[9]: https://www.facebook.com/sharer.php?u=https%3A%2F%2Fwww.r-bloggers.com%2F2017%2F05%2Fthree-way-analysis-of-variance-simple-second-order-interaction-effects-and-simple-main-effects%2F
[10]: https://twitter.com/intent/tweet?text=Three-Way%20Analysis%20of%20Variance%3A%20Simple%20Second-Order%20Interaction%20Effects%20and%20Simple%20Main%20Effects&url=https://www.r-bloggers.com/2017/05/three-way-analysis-of-variance-simple-second-order-interaction-effects-and-simple-main-effects/&via=Rbloggers
[11]: http://www.statistics-with-r.com/
[12]: http://statistics-with-r.com/datasets/vitamin3.csv
[13]: https://feedburner.google.com/fb/a/mailverify?uri=RBloggers
[14]: https://www.r-project.org/ "The R Project for Statistical Computing"
[15]: https://www.r-bloggers.com/how-to-learn-r-2/ "R tutorials"
[16]: https://www.r-users.com/ "Data science jobs"
[17]: https://i1.wp.com/www.r-bloggers.com/wp-content/uploads/2020/07/RBloggers_feedburner_count_2020_07_01-e1593671704447.gif?w=578&#038;ssl=1
[18]: https://feeds.feedburner.com/RBloggers
[19]: https://www.facebook.com/rbloggers/
[20]: https://www.r-bloggers.com/2018/07/pca-vs-autoencoders-for-dimensionality-reduction/ "PCA vs Autoencoders for Dimensionality Reduction"
[21]: https://www.r-bloggers.com/2016/11/5-ways-to-subset-a-data-frame-in-r/ "5 Ways to Subset a Data Frame in R"
[22]: https://www.r-bloggers.com/2015/12/how-to-write-the-first-for-loop-in-r/ "How to write the first for loop in R"
[23]: https://www.r-bloggers.com/2022/03/how-to-calculate-a-cumulative-average-in-r/ "How to Calculate a Cumulative Average in R"
[24]: https://www.r-bloggers.com/2013/08/date-formats-in-r/ "Date Formats in R"
[25]: https://www.r-bloggers.com/2022/03/complete-tutorial-on-using-apply-functions-in-r/ "Complete tutorial on using &#039;apply&#039; functions in R"
[26]: https://www.r-bloggers.com/2010/02/r-sorting-a-data-frame-by-the-contents-of-a-column/ "R – Sorting a data frame by the contents of a column"
[27]: https://www.r-bloggers.com/2022/03/something-to-note-when-using-the-merge-function-in-r/
[28]: https://www.r-bloggers.com/2022/03/better-sentiment-analysis-with-sentiment-ai/
[29]: https://www.r-bloggers.com/2022/03/self-documenting-plots-in-ggplot2/
[30]: https://www.r-bloggers.com/2022/03/data-challenges-for-r-users/
[31]: https://www.r-bloggers.com/2022/03/simplevis-new-improved/
[32]: https://www.r-bloggers.com/2022/03/checking-the-inputs-of-your-r-functions/
[33]: https://www.r-bloggers.com/2022/03/imputing-missing-values-in-r/
[34]: https://www.r-bloggers.com/2022/03/creating-a-dashboard-framework-with-aws-part-1/
[35]: https://www.r-bloggers.com/2022/03/bensstatstalks3-5-tips-for-landing-a-data-professional-role/
[36]: https://www.r-bloggers.com/2022/03/live-covid-19-swiss-vaccination-analysis/
[37]: https://www.r-bloggers.com/2022/03/complete-tutorial-on-using-apply-functions-in-r/
[38]: https://www.r-bloggers.com/2022/03/getting-to-know-julia/
[39]: https://www.r-bloggers.com/2022/03/bootstraps-bandings/
[40]: https://www.r-bloggers.com/2022/03/how-to-calculate-a-cumulative-average-in-r/
[41]: https://www.r-bloggers.com/2022/03/some-thoughts-about-the-use-of-cloud-services-and-web-apis-in-social-science-research/
[42]: https://feeds.feedburner.com/Rjobs
[43]: https://www.r-users.com/
[44]: https://www.r-users.com/jobs/junior-data-scientist-quantitative-economist/
[45]: https://www.r-users.com/jobs/senior-quantitative-analyst/
[46]: https://www.r-users.com/jobs/r-programmer-4/
[47]: https://www.r-users.com/jobs/data-scientist-cgiar-excellence-in-agronomy-ref-no-ddg-r4d-ds-1-cg-ea-06-20/
[48]: https://www.r-users.com/jobs/data-analytics-auditor-future-of-audit-lead-london-or-newcastle/
[49]: https://feeds.feedburner.com/Python-bloggers
[50]: https://python-bloggers.com/
[51]: https://python-bloggers.com/2022/03/dunn-index-for-k-means-clustering-evaluation/
[52]: https://python-bloggers.com/2022/03/installing-python-and-tensorflow-with-jupyter-notebook-configurations/
[53]: https://python-bloggers.com/2022/03/how-to-get-twitter-data-using-python/
[54]: https://python-bloggers.com/2022/02/visualizations-with-altair/
[55]: https://python-bloggers.com/2022/02/spelling-corrector-program-in-python/
[56]: https://python-bloggers.com/2022/02/spelling-checker-program-in-python/
[57]: https://python-bloggers.com/2022/02/streamlit-tutorial-how-to-deploy-streamlit-apps-on-rstudio-connect/
[58]: https://www.r-bloggers.com/blogs-list/
[59]: http://www.proc-x.com/ "SAS news gathered from bloggers"
[60]: https://www.mhthemes.com/
[61]: #
