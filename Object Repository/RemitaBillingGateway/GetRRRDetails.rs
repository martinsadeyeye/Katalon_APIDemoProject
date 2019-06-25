<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GetRRRDetails</name>
   <tag></tag>
   <elementGuidId>4df07572-e280-4afd-a258-9a6bce5a6677</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>publicKey</name>
      <type>Main</type>
      <value>U0hFTEx8MTUwOTM3MTg1NDczOXwyNjdjNDBiZmI5ZjMzMjg5M2I3MWI2YzEzZWUxYTQ5YjUxOTRhMjY5ZDljOWUzNmI0MWUxOTgyYzI1NDUyYTMxM2NlM2QxYTdmZjQxMTExN2M5MTU1NjgxNWYyYmEwMTI3ZWY3MmU4M2MxNmE2ZjBmNjE3Y2Q2OTNlYzA1ODA4Nw==</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>transactionId</name>
      <type>Main</type>
      <value>23</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>http://192.9.200.209:6200/microservice/remita/gateway/send/api/bgatesvc/billing/lookup/140216546698</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()



WS.verifyElementPropertyValue(response, 'responseData[0].amountDue', '10157.5')


WS.verifyElementPropertyValue(response, 'responseData[0].currency', 'NGN')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
